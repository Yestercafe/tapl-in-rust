use crate::syntax::T;

struct Parser<'a> {
    i: usize,
    pub tokens: Vec<&'a str>,
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<&str>) -> Parser {
        Parser { i: 0, tokens }
    }

    fn get_next_term(&mut self) -> Result<T, String> {
        let i = self.i;
        if i == self.tokens.len() {
            return Err("Parser error: wrong syntax".into());
        }
        self.i += 1;
        match self.tokens[i] {
            "true" => Ok(T::True),
            "false" => Ok(T::False),
            "if" => Ok(T::If(
                Box::new(self.get_next_term().expect("Parser error: wrong token")),
                Box::new(self.get_next_term().expect("Parser error: wrong token")),
                Box::new(self.get_next_term().expect("Parser error: wrong token")),
            )),
            "then" => self.get_next_term(),
            "else" => self.get_next_term(),
            _ => Err("Parser error: wrong token".into())
        }
    }
}

fn parse_tokens(parser: Parser) -> Result<T, String> {
    let mut parser = parser;
    return parser.get_next_term()
}

pub fn parse(expr: &str) -> T {
    let tokens = expr.split_whitespace().collect::<Vec<&str>>();
    let ret = parse_tokens(Parser::new(tokens)).unwrap();
    println!("{:?}", ret);
    ret
}
