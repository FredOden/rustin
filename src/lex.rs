use regex::Regex;
use serde::Deserialize;

#[derive(Debug)]
pub struct Token {
    token: String,
    string: String,
    start: usize,
    end: usize
}

#[derive(Deserialize, Debug)]
struct Lexicon {
    tokens:std::collections::BTreeMap<String,String>
}

pub fn lex(json_lexicon: String, text: String) -> Result<Vec<Token>, String> {
    let mut scratch = text.clone();
    let mut tokens:Vec<Token> = Vec::new();
    match serde_json::from_str::<Lexicon>(&json_lexicon) {
        Ok(lexicon) => {
            for (k, tok) in lexicon.tokens {
                let re = Regex::new(&tok).unwrap();
                let sc = scratch.clone();

                let at:Vec<regex::Match> = re.find_iter(&sc).collect();

                for m in &at {
                    let tk =Token{
                        token:k.clone(), 
                        //at: m.clone() 
                        string: m.as_str().to_string(),
                        start: m.start(),
                        end: m.end()
                    };

                    tokens.push(tk);
                    scratch.replace_range(m.start() .. m.end() , &" ".repeat(m.end() - m.start()));
                }
            }
            tokens.sort_by(|a, b| a.start.cmp(&b.start));
            let re = regex::Regex::new(r"([^\s]+)").unwrap();
            let invalids:Vec<regex::Match> = re.find_iter(&scratch).collect();
            let mut errors: String = "".to_string();
            for invalid in &invalids {
                errors = format!("{}lex invalid \"{}\" at {}\n", errors, invalid.as_str(), invalid.start());
            }

            if errors.len() > 0 {
                return Err(errors);
            }
        }
        Err(e) => {
            return Err(format!("lex error::{}", e));
        }

    }
    Ok(tokens)
}
