use std::fs;
use structopt::StructOpt;
use std::path::PathBuf;

mod lex;
mod p_code;
mod parser;

#[derive(StructOpt, Debug)]
#[structopt(name = "rustin")]
#[structopt(version = "0.1.2")]
#[structopt(about = "parser based on json descriptions")]
struct Opt {
    /// lexicon
    #[structopt(short, long)]
    lexicon:PathBuf,
    /// grammar
    #[structopt(short, long)]
    grammar:PathBuf,
    /// source code
    #[structopt(short, long)]
    source:PathBuf,
    /// verbose
    #[structopt(short, long)]
    verbose:bool,
    /// rule
    #[structopt(short, long)]
    rule:String,
}

fn main() {
    match Opt::from_args_safe() { 
        Ok(opt) => {                         
            let json = fs::read_to_string(opt.lexicon).unwrap();
            let source = fs::read_to_string(opt.source).unwrap();
            let grammar = fs::read_to_string(opt.grammar).unwrap();
            match lex::lex(json, source) {
                Ok(tokens) => {
                    if opt.verbose { println!("Lex -> {:#?}", tokens); }
                   println!("Can check syntax");
                   let mut  parser = parser::Parser::new(grammar, tokens);
                   parser.parse("top".to_string(), opt.rule, 0);
                }
                Err(e) => {
                    eprintln!("lex failed::{}", e);
                    std::process::exit(1);
                }
            }
        }                                    
        Err(e) => {                
            eprintln!("problem: {}", e);
            std::process::exit(1);
        }
    }
    println!("exit");
}
