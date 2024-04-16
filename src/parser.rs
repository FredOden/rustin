use crate::lex;
use serde::Deserialize;
use std::collections::HashMap;
use std::cmp::Ordering;
use crate::p_code;
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

#[derive(Clone, Copy, Debug)]
pub struct Parsed {
    at: usize,
    count: usize,
    broken: bool,
    // that's all for the moment
}

impl Parsed {
    fn new(at:usize, count:usize) -> Parsed {
        Parsed {
            at,
            count,
            broken: false,
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
}

impl Parser {
    pub fn new(grammar_json: String, tokens: Vec<lex::Token>) -> Parser {
        let grammar:Grammar = match serde_json::from_str::<Grammar>(&grammar_json) {
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
        };

        //println!("grammar::{:#?}", p.grammar);
        for(k, v) in &p.grammar.rules {
            let mut r: Vec<String> = Vec::new();
            for i in v.keys() {
                r.push((*i).to_string());
                let mut vat: Vec<usize> = Vec::new();
                p.forbidden_rules.push(
                    ForbiddenRule::new((*i).to_string())
                );
            }
            r.sort_by(|a,b| b.split(" ").count().cmp(&a.split(" ").count()));
            p.keys.insert((*k.clone()).to_string(), r);
        }
        // println!("keys :: {:#?}", p.keys);
        //println!("forbidden_rules :: {:#?}", p.forbidden_rules);
        p
    }

    fn inc_depth(&self) {
        *self.depth.borrow_mut() += 1;
    }

    fn dec_depth(&self) {
        *self.depth.borrow_mut() -= 1;
        //println!("<<<<<<<{}", self.depth());
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
                return **x;
            }
            None => {
                return None;
            }
        }
    }

    fn is_broken(&self, try_rule : &String, at:usize) -> bool {
        if let Some(p) = self.find_cache(try_rule, at) {
            return p.broken;
        }
        return false;
    }

    fn set_broken(&self, try_rule : &String, at:usize) {
        if let Some(mut p) = self.find_cache(try_rule, at) {
            p.broken = true;
        }
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
        println!("{}::in parse \"{}\" at {}", self.depth(), name, at_start);

        let mut binding = self.keys.get(&name);
        let mut vok = binding.iter_mut();
        'mainLoop: for try_rules in vok {
            'rulesLoop: for try_rule in try_rules.iter() {

                if let Some(fr) = self.find_forbidden(try_rule) {
                    if fr.is_at(at_start) {
                        println!("{}::@@@@ {try_rule} forbidden at {at_start}", self.depth());
                        continue;
                    }
                }

                if self.is_broken(try_rule, at_start) {
                    println!("{}:: @@@@ {try_rule} BROKEN at {at_start}", self.depth());
                    continue;
                }

                if let Some(s) = self.find_cache(try_rule, at_start) {
                    println!("{}::@@@@ Cached rule {try_rule} at {at_start} -> {s:#?}", self.depth());
                    self.dec_depth();
                    return Some(s);
                }

                let s_syntax = try_rule.split(" ");

                let mut syntax:Vec<String> = Vec::new();
                for s in s_syntax {
                    syntax.push(s.to_string());
                }

                let count = syntax.len();
                println!("{}:: try_rule {try_rule} :: {count} at {at_start}", self.depth(),);
                if at_start + count -1 >= self.tokens.len() {
                    println!("{}:: @@@@ END OF TOKENS rule <{try_rule}> too long", self.depth());
                    continue;
                }
                let mut at = at_start;
                'syntax: for i_element in 0..syntax.len() {
                    let element:String = syntax[i_element].clone();
                    if at + i_element >= self.tokens.len() {
                        println!("{}:: @@@@ END OF TOKENS", self.depth());
                        continue 'rulesLoop;
                    }
                    println!("{}:: syntax[{i_element}] {element} <- {}[{}]", self.depth(), self.tokens[at + i_element].string(), at + i_element);
                    if self.tokens[at + i_element].token().cmp(&element) != Ordering::Equal {
                        if element[0..1].cmp("&") == Ordering::Equal {
                            let sub_rule = &element[1..];
                            if let Some(fr) = self.find_forbidden(try_rule) {
                                fr.add_at(at + i_element);
                            }
                            let mut  opt_parsed = self.parse("".to_string(), sub_rule.to_string(), at + i_element);
                            println!("{}: @@@@ opt_parsed :{opt_parsed:?} {sub_rule} {at} {i_element}", self.depth());
                            if let Some(fr) = self.find_forbidden(try_rule) {
                                fr.remove_at(at + i_element);
                            }
                            if let  Some(parsed) =  opt_parsed {
                                let at_before = at;
                                at = parsed.at + parsed.count -1 - i_element;
                                println!("{}::@@@@ Partial Match {try_rule}[{i_element}]<{element}> & before {}<{}> now at {}<{}>", self.depth(), at_before +i_element, self.tokens[at_before + i_element].string(), at + i_element, self.tokens[at + i_element].string());
                                //continue 'syntax;
                            } else {
                                println!("{}::@@@@ Break {try_rule} & at {} on {}::{} {i_element}/{count}", self.depth(), at, element, i_element);
                                self.set_broken(try_rule, at_start);
                                break 'syntax;
                            }
                            //continue 'syntax;
                        } else {
                            // here element not matched
                            // process next rule
                            println!("{}::@@@@ Breaking {try_rule} & at {} on {}::{} {i_element}/{count}'", self.depth(), at, element, i_element);
                            self.set_broken(try_rule, at_start);
                            break 'syntax;
                        }
                        println!("{}::grey zone {try_rule} {i_element} {at}", self.depth());
                    }
                    // store p_code for simpke token
                    if i_element == count - 1 {
                        println!("{}::@@@@ Matched {try_rule} {i_element} & at {}", self.depth(), at + i_element );
                        let mut op = Some(Parsed::new(at, count));
                        self.insert_cache(try_rule, at_start, op);
                        self.dec_depth();
                        return op;
                    }
                }
            }
        }
        self.dec_depth();
        return None;
    }

}
