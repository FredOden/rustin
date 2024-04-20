use std::fs;
use structopt::StructOpt;
use std::path::PathBuf;

mod lex;
mod parser;
mod compiler;

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
    /// output compiled code
    #[structopt(short, long)]
    output:PathBuf,
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
            let mut next:std::option::Option<parser::Parsed>;
            match lex::lex(json, source) {
                Ok(tokens) => {
                    if opt.verbose { println!("Lex -> {:#?}", tokens); }
                   println!("Can check syntax");
                   let mut  parser = parser::Parser::new(grammar, tokens);
                   next = parser.parse("top".to_string(), opt.rule, 0);
                   if let Some(parsed) = next {
                       println!("END PARSING AT::{}", parsed.at);
                       compiler::compile(
                           parsed,
                           opt.output
                       );
                   }
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
