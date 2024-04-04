use regex::Regex;
//use serde::Deserialize

pub fn is_matching(s:String, re:String) -> bool {
    false
}

pub fn trying() {
    println!(">>>>>trying in");
    let re = Regex::new(r"(?P<word>[a-z]*)+").unwrap();
    let sentence = "zztop goes to holywood";
    println!("     re::{:#?}", re);
    println!("     re.capture::{:#?}", re.captures(sentence));
    let words: Vec<&str> = re.find_iter(sentence).map(|m| {
        println!("          m::{:#?}", m);
        m.as_str()
    }).collect();
    println!("     words::{:#?}", words);
    /*
     *
     *
    let l = " gfg hhu".to_string();
    let r = l.replace(r"\s+gfg", "000");
    println!("     r::{:#?}", r);
    */
}
