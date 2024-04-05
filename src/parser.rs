use crate::lex;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct Grammar {
    rules:std::collections::HashMap<String,std::collections::HashMap<String, String>>
}

#[derive(Debug)]
pub struct Parser {
    tokens:Vec<lex::Token>,
    grammar_json: String,
    keys: std::collections::HashMap<String, Vec<String>>,
}

impl Parser {
    pub fn new(grammar_json: String, tokens: Vec<lex::Token>) -> Parser {
        Parser {
            tokens,
            grammar_json,
            keys: HashMap::new(),
        }
    }

    pub fn parse(& mut self, rule: String, name: String, at:usize) {

        match serde_json::from_str::<Grammar>(&self.grammar_json) {
            Ok(grammar) => {
                println!("grammar::{:#?}", grammar);
                ///
                /// sorting each rules
                /// from the longest rule
                /// to the shortest
                ///
                let mut r: Vec<String> = Vec::new();
                for(k, v) in grammar.rules {
                    for i in v.keys() {
                        r.push((*i.clone()).to_string());
                    }
                    let mut rc = r.clone();
                    rc.sort_by(|a,b| b.split(" ").count().cmp(&a.split(" ").count()));
                    self.keys.insert(k, rc);
                }
                println!("keys :: {:#?}", self.keys);
            }
            Err(e) => {
                eprintln!("parse error: {}", e);
            }
        }
    }
}
