use crate::lex;
use serde::Deserialize;
use std::collections::HashMap;
use std::cmp::Ordering;
use crate::p_code;
use std::cell::RefCell;
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

#[derive(Debug)]
pub struct Parser {
    tokens:Vec<lex::Token>,
    grammar_json: String,
    grammar: Grammar,
    keys: std::collections::HashMap<String, Vec<String>>,
    forbidden_rules: Vec<ForbiddenRule>, //HashMap<String, Vec<usize>>,
    depth:RefCell<usize>,
    cache:RefCell<HashMap<CacheKey, usize>>,
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
        println!("<<<<<<<{}", self.depth());
    }

    fn depth(&self) -> usize {
        *self.depth.borrow()
    }

    fn insert_cache(&self, try_rule:&String, at:usize, skip:usize) {
        self.cache.borrow_mut().insert(
            CacheKey::K(try_rule.clone(), at),
            skip,
        );
    }

    fn find_cache(&self, try_rule: &String, at:usize) -> Option<usize> {
       match self.cache.borrow().get(
             &CacheKey::K(try_rule.clone(), at)
             ) {
           Some(pu) => {
               return Some(*pu);
           }
           None => {
               return None;
           }
       }
       return None;
    }


    fn find_forbidden(&self, rule: &String) -> Option<&ForbiddenRule> {
        for fr in self.forbidden_rules.iter() {
            if fr.rule.cmp(&rule) == Ordering::Equal {
                return Some(fr);
            }
        }
        return None;
    }

    pub fn parse(&self, rule: String, name: String, mut at:usize) -> usize {

        self.inc_depth();
        println!("{}::in parse \"{}\" at {}", self.depth(), name, at);

        let mut binding = self.keys.get(&name);
        let mut vok = binding.iter_mut();
        //println!("{}::vok::{:#?}", self.depth(), vok);
        'mainLoop: for try_rules in vok {
            //println!("{}::try_rules::{:#?}", self.depth(), try_rules);
            'rulesLoop: for try_rule in try_rules.iter() {
                if let Some(s) = self.find_cache(try_rule, at) {
                    println!("{}::@@@@ Cached rule {try_rule} at {at} -> {s}", self.depth());

                    self.dec_depth();
                    return s;
                }
                if let Some(fr) = self.find_forbidden(try_rule) {
                    if fr.is_at(at) {
                        println!("{}::@@@@ {try_rule} forbidden at {at}", self.depth());
                        continue;
                    }
                }


                let syntax = try_rule.split(" ");
                let count = syntax.clone().count();
                println!("{}:: try_rule {} :: {}",
                    self.depth(),
                    try_rule,
                    count
                );
                let mut i_element:usize = 0;
                if at + count -1 >= self.tokens.len() {
                        println!("{}:: @@@@ END OF TOKENS rule <{try_rule}> too long", self.depth());
                        continue;
                }
                'syntax: for element in syntax {
                    if at + i_element >= self.tokens.len() {
                        println!("{}:: @@@@ END OF TOKENS", self.depth());
                        continue 'rulesLoop;
                    }
                    println!("{}:: syntax[{i_element}] {element}", self.depth());
                    if self.tokens[at + i_element].token().cmp(&element.to_string()) == Ordering::Equal {
                        if i_element == count - 1 {
                            println!("{}::@@@@ Matched {try_rule}", self.depth());

                            self.dec_depth();
                            return at + count;
                        }
                        i_element += 1;
                        continue 'syntax;
                    } else {
                        if element[0..1].cmp("&") == Ordering::Equal {
                            let sub_rule = &element[1..];
                            if let Some(fr) = self.find_forbidden(try_rule) {
                                fr.add_at(at + i_element);
                            }
                            let skip = self.parse("".to_string(), sub_rule.to_string(), at + i_element);
                            self.insert_cache(try_rule, at, skip);
                            
                            if let Some(fr) = self.find_forbidden(try_rule) {
                                fr.remove_at(at + i_element);
                            }

                            if skip > 0 {
                                at = skip - 1 -i_element;
                                if i_element == count - 1 {
                                    println!("{}::@@@@ Matched {try_rule} &", self.depth());

                                    self.dec_depth();
                                    return at +count ;
                                }
                                i_element += 1;
                                continue 'syntax
                            }
                            continue 'rulesLoop;
                        }
                        // here element not matched
                        // process next rule
                        continue 'rulesLoop
                    }
                    continue 'rulesLoop
                }
            }
        }
        self.dec_depth();
        return 0;
    }

}
