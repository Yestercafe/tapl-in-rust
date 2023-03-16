mod eval;
mod parser;
mod syntax;

use eval::eval;
use parser::parse;
use std::io::stdin;

fn main() {
    let mut expr = String::new();
    stdin().read_line(&mut expr).expect("Input error");
    let t = parse(&expr);
    let result = eval(t).unwrap();
    println!("{:?}", result);
}
