use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Token {
    token: String,
    string: String,
    start: usize,
    end: usize,
    line : usize,
    column: usize
}

impl Token {
    pub fn start(&self) -> usize { self.start }
    pub fn line(&self) -> usize { self.line }
    pub fn column(&self) -> usize { self.column }
    pub fn end(&self) -> usize { self.end }
    pub fn string(&self) -> &String { &self.string }
    pub fn token(&self) -> &String { &self.token }
}

#[derive(Deserialize, Debug)]
struct Lexicon {
    tokens:std::collections::BTreeMap<String,String>
}

///
/// lex - a lexical parser based on json
/// description
///
/// parameters:
///     json_lexicon : json description
///     text : source code to analyse
///
/// returns Result<Vec<Token>, String>
///
/// json description is stored in a BTreeMap
/// ```
/// {
/// "tokens": {
///       "token" : "regular expression",
///       ...
///       }
/// }
/// ```
///example: 
///
/// ```
///{
///       "tokens" :{
///		";" : ";",
///		"@equals" : "=",
///		"@if": "\\bif\\b",
///		"@then" : "\\bthen\\b",
///		"@else" : "\\belse\\b",
///		"@Number" : "\\b((?:0x[0-9a-f]+)|(?:\\d*[.]?\\d+(?:(?:[E|e][\\+\\-]?)\\d*[.]?\\d+)?))\\b",
///		"Identifier" : "([A-Za-z_][0-9A-Za-z_]*)"
///	}
///}
///-------- end of file
/// ```
///
/// issues:
/// the "general" tokens like identifiers
/// have to be processed at last
/// after keywords or other kind of numbers
/// like integer for example
/// the idea is to prefix keywords with @
/// (let's think about it).
///
/// so for a source code like this
///-------- begining of file
/// ```
///if zz and e45 then top = 54.6e+6
///else if 14;
/// ```
///-------- end of file
///
/// lex will return Ok(tokens) where tokens
/// is vector of Token. This vector is
/// ordered following th position of
/// the token in the file, preparing for the
/// syntax checking.
///
/// 
/**
Lex -> [
    Token {
        token: "@if",
        string: "if",
        start: 0,
        end: 2,
    },
    Token {
        token: "Identifier",
        string: "zz",
        start: 3,
        end: 5,
    },
    Token {
        token: "Identifier",
        string: "and",
        start: 6,
        end: 9,
    },
    Token {
        token: "Identifier",
        string: "e45",
        start: 10,
        end: 13,
    },
    Token {
        token: "@then",
        string: "then",
        start: 14,
        end: 18,
    },
    Token {
        token: "Identifier",
        string: "top",
        start: 19,
        end: 22,
    },
    Token {
        token: "@equals",
        string: "=",
        start: 23,
        end: 24,
    },
    Token {
        token: "@Number",
        string: "54.6e+6",
        start: 25,
        end: 32,
    },
    Token {
        token: "@else",
        string: "else",
        start: 33,
        end: 37,
    },
    Token {
        token: "@if",
        string: "if",
        start: 38,
        end: 40,
    },
    Token {
        token: "@Number",
        string: "14",
        start: 41,
        end: 43,
    },
    Token {
        token: ";",
        string: ";",
        start: 43,
        end: 44,
    },
]
Can check syntax
exit
**/
///
pub fn lex(json_lexicon: String, text: String) -> Result<Vec<Token>, String> {
    let mut scratch = text.clone();
    let mut tokens:Vec<Token> = Vec::new();
    match deser_hjson::from_str::<Lexicon>(&json_lexicon) {
        Ok(lexicon) => {
            for (k, tok) in lexicon.tokens {
                let re = Regex::new(&tok).unwrap();
                let sc = scratch.clone();

                let at:Vec<regex::Match> = re.find_iter(&sc).collect();

                for m in &at {
                    let (line, column) = rustin::line_col(text.clone(), m.start());
                    let tk =Token{
                        token:k.clone(), 
                        string: m.as_str().to_string(),
                        start: m.start(),
                        end: m.end(),
                        line ,
                        column
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
                let (line, column) = rustin::line_col(text.clone(), invalid.start());
                errors = format!("{}lex invalid \"{}\" at line:{} col:{} index:{}\n", errors, invalid.as_str(), line, column, invalid.start());
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
