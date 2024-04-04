use std::fs;
use structopt::StructOpt;
use std::path::PathBuf;

mod lex;

#[derive(StructOpt, Debug)]
#[structopt(name = "rustin")]
#[structopt(version = "0.1.2")]
#[structopt(about = "Pippo evaluates rust")]
struct Opt {
    /// lexicon
    #[structopt(short, long)]
    lexicon:PathBuf,
    /// source code
    #[structopt(short, long)]
    source:PathBuf,
}

fn main() {
    match Opt::from_args_safe() { 
        Ok(opt) => {                         
            let json = fs::read_to_string(opt.lexicon).unwrap();
            let source = fs::read_to_string(opt.source).unwrap();
            match lex::lex(json, source) {
                Ok(tokens) => {
                    println!("Lex -> {:#?}", tokens);
                    println!("Can check syntax");
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
