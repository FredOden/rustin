use crate::lex;
use serde::Deserialize;
use std::collections::HashMap;
use std::cmp::Ordering;
use crate::p_code;
use std::cell::RefCell;


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
    pub fn add(&self, it:usize) {
        self.at.borrow_mut().push(it);
    }
}

#[derive(Debug)]
pub struct Parser {
    tokens:Vec<lex::Token>,
    grammar_json: String,
    grammar: Grammar,
    keys: std::collections::HashMap<String, Vec<String>>,
    forbidden_rules: Vec<ForbiddenRule> //HashMap<String, Vec<usize>>,
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

    pub fn parse(&self, rule: String, name: String, mut atStart:usize) -> bool{

        println!("in parse {} at {}", name, atStart);

        let mut binding = self.keys.get(&name);
        let mut vok = binding.iter_mut();
        'mainLoop: for tryRules in vok {
            'rulesLoop: for tryRule in tryRules.iter() {

                for i in 0..self.forbidden_rules.len() {
                    let mut fr = &self.forbidden_rules[i];
                    if fr.rule.cmp(&tryRule) == Ordering::Equal {
                        for fat in fr.at.borrow().iter() {
                            if fat.cmp(&& mut atStart) == Ordering::Equal {
                                println!("::::: {} forbidden at {} leaving ...", tryRule, fat);
                                //continue 'rulesLoop;
                                return false;
                            }
                        }
                        fr.add(atStart);
                    }
                }

                let syntax = tryRule.split(" ");
                let count = syntax.clone().count();
                let mut at = atStart;
                let mut iElement:usize = 0;
                for element in syntax {
                    //println!(" element[{}]::{}", iElement, element);

                    /*println!("token[{}]::{} (\"{}\") -- element[{}]::{}",
                        at + iElement,
                        self.tokens[at + iElement].token(),
                        self.tokens[at + iElement].string(),
                        iElement,
                        element,
                        );
                    */
                    if self.tokens[at + iElement].token().cmp(&element.to_string()) == Ordering::Equal {
                        println!("  >>>>>>skip {}", element);
                        iElement += 1;
                        if iElement == count { 
                            println!("!!!!!!!! MATCHED {}", tryRule);
                            return true; 
                        }
                        continue;
                    }
                    if element[0..1].cmp("&") == Ordering::Equal {
                        let r = &element[1..];
                        println!("      ......will go in {}/{}", r, name);
                        if name.cmp(&r.to_string()) != Ordering::Equal || atStart != (at + iElement) {
                            if self.parse(rule.clone(), r.to_string(), at + iElement) {
                                iElement += 1;
                                if iElement == count {
                                    println!("******** MATCHED {}", tryRule);
                                    return true;
                                }
                                continue;
                            }
                        } else {
                            continue 'rulesLoop;
                        }
                    } else {
                        continue 'rulesLoop;
                    }
                    iElement += 1;
                }
            }
        }
        return false;
    }

}
