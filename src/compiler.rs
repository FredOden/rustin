use crate::parser;
use std::path::PathBuf;
use std::io::Write;



pub fn compile(root: parser::Parsed, output: PathBuf) -> Result<String, String> {

    println!("compiling");
    let mut sout:String = String::new();
    sout.push_str(root.val.as_str());

    eval(&mut sout, root.p__);


    match  std::fs::File::create(output) {
        Err(e) => {
            return Err(format!("compile ERROR File::create::{e}"));
        }
        Ok(mut file) => {
            if let Err(e) = file.write_all(sout.as_bytes()) {
                return Err(format!("compile ERROR File::write_all::{e}"));
            }
        }
    }
    
    Ok(format!("compilation done."))
}


fn eval(val:&mut String, p__:Vec<parser::Parsed>) {
    for i in 0..p__.len() {
        let p = p__[i].clone();
        let mut v = p.val;
        eval(&mut v, p.p__);
        let pattern = format!("$({i})");
        *val = val.replace(pattern.as_str(), v.as_str());
    };
    //val.push('\n');
}
