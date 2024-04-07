use serde::Deserialize;
use crate::lex;

#[derive(Deserialize, Clone, Debug)]
pub enum PCode {
    Nothing(),
    Error(String),
    Identifier(String),
    Bool(bool),
    Number(String),
    CharString(String),
    If(Box<Element>, Box<Element>, Box<Element>),
    Op(String, Box<Element>, Box<Element>),
    Unary(String, Box<Element>),
    Loop(Box<Element>, Box<Element>),
    Function(String, Vec<Box<Element>>, Box<Element>),
    Call(String, Vec<Box<Element>>),
}

#[derive(Deserialize, Clone, Debug)]
pub struct Element {
    p_code: PCode,
    token: lex::Token
}


///
/// eval - evaluates PCode
///
/// like
/// 
/// eval(If(Bool(a == b), Do(..then..), Do(..else..)))
///
pub fn eval(p: Element) -> Result<Element, String> {
    match p.p_code {
        PCode::If(cond, p_then, p_else) => {
            match eval(*cond.clone()).unwrap().p_code {
                PCode::Bool(true) => {
                    return eval(*p_then);
                }
                PCode::Bool(false) => {
                    return eval(*p_else);
                }
                _ => {
                    return Err(format!("not a boolean condition at {}", cond.token.start()));
                }
            }
        }
        PCode::Unary(operation, p) => {
            return Err(format!("Unary not implemented"));
        }
        PCode::Op(operation, p_left, p_right) => {
            return Err(format!("Op not implemented"));
        }
        PCode::Loop(cond, body) => {
            while let PCode::Bool(true) = eval(*cond.clone()).unwrap().p_code {
                let _ = eval(*body.clone());
            }
            return Ok(Element{
                p_code: PCode::Nothing(),
                token: p.token
            });
        }
        PCode::Function(name, params, body) => {
            return Err(format!("Function not implemented"));
        }
        PCode::Call(name, params) => {
            return Err(format!("Call not implemented"));
        }
        _ => {
            return Ok(p);
        }
    }
}
