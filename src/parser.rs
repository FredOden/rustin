use crate::lex;
use serde::Deserialize;
use deser_hjson::*;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::cell::RefCell;
use std::rc::Rc;
use std::option::Option;

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum CacheKey {
    K(String, usize),
}

#[derive(Deserialize, Clone, Debug)]
struct Grammar {
    rules:std::collections::HashMap<String,std::collections::HashMap<String, String>> 
}

pub fn push(v: & mut Vec<usize>, i:usize) {
    v.push(i);
}

#[derive(Debug)]
struct ForbiddenRule {
    rule: String,
    at: RefCell<Vec<usize>>
}

impl ForbiddenRule {
    pub fn new(rule:String) -> ForbiddenRule {
        ForbiddenRule {
            rule,
            at: RefCell::new(Vec::new())
        }
    }
    pub fn add_at(&self, it:usize) {
        self.at.borrow_mut().push(it);
    }

    pub fn is_at(&self, at:usize) -> bool {
        for it in self.at.borrow().iter() {
            if it.cmp(&at) == Ordering::Equal {
                return true;
            }
        }
        return false;
    }

    pub fn remove_at(&self, at:usize) {
        let mut i:usize = 0;
        for it in self.at.borrow().iter() {
            if it.cmp(&at) == Ordering::Equal {
                break;
            }
            i += 1;
        }
        self.at.borrow_mut().remove(i);
    }
}

#[derive(Clone, Debug)]
pub struct Parsed {
    pub at: usize,
    count: usize,
    // that's all for the moment
    pub p__: Vec<Parsed>,
    pub val: String
}

impl Parsed {
    fn new(at:usize, count:usize, p__: Vec<Parsed>, val:String) -> Parsed {
        Parsed {
            at,
            count,
            p__,
            val,
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    tokens:Vec<lex::Token>,
    grammar_json: String,
    grammar: Grammar,
    keys: std::collections::HashMap<String, Vec<String>>,
    forbidden_rules: Vec<ForbiddenRule>, //HashMap<String, Vec<usize>>,
    depth:RefCell<usize>,
    cache:RefCell<HashMap<CacheKey, Option<Parsed>>>,
    broken:RefCell<HashMap<CacheKey, bool>>,
}

impl Parser {
    pub fn new(grammar_json: String, tokens: Vec<lex::Token>) -> Parser {
        let grammar:Grammar = match deser_hjson::from_str::<Grammar>(&grammar_json) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("parse error: {}", e);
                Grammar {
                    rules:HashMap::new()
                }
            }
        };

        let mut p = Parser {
            tokens,
            grammar_json,
            grammar:grammar,
            keys: HashMap::new(),
            forbidden_rules : Vec::new(),
            depth:RefCell::new(0),
            cache:RefCell::new(HashMap::new()),
            broken:RefCell::new(HashMap::new()),
        };

        for(k, v) in &p.grammar.rules {
            let mut r: Vec<String> = Vec::new();
            for i in v.keys() {
                r.push((*i).to_string());
                //let mut vat: Vec<usize> = Vec::new();
                p.forbidden_rules.push(
                    ForbiddenRule::new((*i).to_string())
                );
            }
            r.sort_by(|a,b| b.split(" ").count().cmp(&a.split(" ").count()));
            p.keys.insert((*k.clone()).to_string(), r);
        }
        p
    }

    fn inc_depth(&self) {
        *self.depth.borrow_mut() += 1;
    }

    fn dec_depth(&self) {
        *self.depth.borrow_mut() -= 1;
    }

    fn depth(&self) -> usize {
        *self.depth.borrow()
    }

    fn insert_cache(&self, try_rule:&String, at:usize, opt_parsed:Option<Parsed>) {
        self.cache.borrow_mut().insert(
            CacheKey::K(try_rule.clone(), at),
            opt_parsed,
        );
    }

    fn find_cache(&self, try_rule: &String, at:usize) -> Option<Parsed> {
        match &self.cache.borrow().get(
            &CacheKey::K(try_rule.clone(), at)
        )
        {
            Some(x) => {
                let op =  *x;
                match op {
                    None => { return None; }
                    Some(p) => {
                        return Some(Parsed {
                            at: p.at,
                            count: p.count,
                            p__: p.p__.clone(),
                            val: p.val.clone(),
                        });
                    }
                }
            }
            None => {
                return None;
            }
        }
    }

    fn is_broken(&self, try_rule : &String, at:usize) -> bool {
        match self.broken.borrow().get(
            &CacheKey::K(try_rule.clone(), at)
        ) {
            Some(b) => {
                return *b;
            }
            None => {
                return false;
            }
        }
    }

    fn line_col(&self, at:usize) -> String {
        return format!("line {} col {}",
            self.tokens[at].line(),
            self.tokens[at].column()
        );
    }

    fn set_broken(&self, try_rule : &String, at:usize) {
        self.broken.borrow_mut().insert(
            CacheKey::K(try_rule.clone(), at),
            true,
        );
    }


    fn find_forbidden(&self, rule: &String) -> Option<&ForbiddenRule> {
        for fr in self.forbidden_rules.iter() {
            if fr.rule.cmp(&rule) == Ordering::Equal {
                return Some(fr);
            }
        }
        return None;
    }

    pub fn parse(&self, rule: String, name: String, at_start:usize) -> Option<Parsed> {

        self.inc_depth();
        const TERMINAL:&str = "!Terminal:";
        const EXPECT:&str = "!Expect:";

        let mut terminal = false;

        let mut binding = self.keys.get(&name);

        if let None = binding {
            binding = self.keys.get(&format!("{TERMINAL}{name}"));
            if let Some(ok) = binding {
                terminal = true;
                println!("{}::!!!! {name} is terminal", self.depth());
            }

        }
        let mut at = at_start;
        let mut vok = binding.iter_mut();
        'mainLoop: for try_rules in vok {
            'rulesLoop: for try_rule in try_rules.iter() {

                if let Some(fr) = self.find_forbidden(try_rule) {
                    if fr.is_at(at_start) {
                        continue;
                    }
                }

                if self.is_broken(try_rule, at_start) {
                    continue;
                }

                if let Some(s) = self.find_cache(try_rule, at_start) {
                    self.dec_depth();
                    return Some(s);
                }

                let s_syntax = try_rule.split(" ");

                let mut syntax:Vec<String> = Vec::new();
                for s in s_syntax {
                    syntax.push(s.to_string());
                }

                let count = syntax.len();
                if at_start + count -1 >= self.tokens.len() {
                    continue;
                }
                at = at_start;
                let mut p__:Vec<Parsed> = Vec::new();
                'syntax: for i_element in 0..syntax.len() {
                    let mut element:String = syntax[i_element].clone();
                    let mandatory =  element.len() > EXPECT.len() &&  element[0..EXPECT.len()].cmp(EXPECT) == Ordering::Equal;
                    if mandatory {
                        element = element[EXPECT.len()..].to_string();
                        println!("{}::!!!! {try_rule}:: mandatory::{mandatory} element::<{element}>", self.depth());
                    }

                    if at + i_element >= self.tokens.len() {
                        continue 'rulesLoop;
                    }
                    if self.tokens[at + i_element].token().cmp(&element) != Ordering::Equal {
                        if element[0..1].cmp("&") == Ordering::Equal {
                            let sub_rule = &element[1..];
                            if let Some(fr) = self.find_forbidden(try_rule) {
                                fr.add_at(at + i_element);
                            }
                            let mut  opt_parsed = self.parse("".to_string(), sub_rule.to_string(), at + i_element);
                            if let Some(fr) = self.find_forbidden(try_rule) {
                                fr.remove_at(at + i_element);
                            }
                            if let  Some(parsed) =  opt_parsed {
                                let at_before = at;
                                at = parsed.at + parsed.count -1 - i_element;
                                p__.push(parsed);
                            } else {
                                self.set_broken(try_rule, at_start);
                                if mandatory {
                                    eprintln!("!!!! {name}:: Syntax error at {} {element} expected", self.line_col(at + i_element));
                                }
                                break 'syntax;
                            }
                        } else {
                            // here element not matched
                            // process next rule
                            self.set_broken(try_rule, at_start);

                            if mandatory {
                                eprintln!("!!!! {name}:: Syntax error at {} {element} expected", self.line_col(at + i_element));
                            }
                            break 'syntax;
                        }
                    } else {
                        p__.push(Parsed{
                            at: at + i_element,
                            count: 1,
                            p__: Vec::new(),
                            val: self.tokens[at + i_element].string().clone(),
                        });
                    }
                    // store p_code for simpke token
                    if i_element == count - 1 {
                        let val = if terminal {
                            self.grammar.rules.get(&format!("{TERMINAL}{name}")).unwrap().get(try_rule).unwrap().to_string()
                        } else {
                            self.grammar.rules.get(&name).unwrap().get(try_rule).unwrap().to_string()
                        };
                        println!("{}::@@@@ Matched {try_rule} {i_element} & at {} ==> {val}", self.depth(), at + i_element );
                        let mut op = Some(Parsed{at, count, p__, val});
                        self.insert_cache(try_rule, at_start, op.clone());
                        self.dec_depth();
                        return op;
                    }
                    if i_element == count - 1 {
                        self.dec_depth();
                        return None
                    }
                }
                self.insert_cache(try_rule, at_start, None);
            } // for try_rule in try_rules
        } // for try_rulea in vok
        
         if terminal {
            eprintln!("{}::!!!! {name}:: Syntax error at {} unexpexted \"{}\"",self.depth(), self.line_col(at), self.tokens[at].string());
        }
        
        self.dec_depth();
        return None;
    }

}
