use std::cmp::Ordering;

pub fn line_col(text: String, idx:usize) -> (usize, usize) {
    let (mut line,mut col) = (1,1);
    for c in text[0..idx].chars() {
        if c as usize == 10 {
        //if text.chars(i) == <Char>::"\n" {
            line += 1;
            col =1;
        } else {
            col += 1;
        }
    }   
    (line, col)
}
