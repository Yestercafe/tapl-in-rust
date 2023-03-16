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
                Box::new(self.get_next_term().unwrap()),
                Box::new(self.get_next_term().unwrap()),
                Box::new(self.get_next_term().unwrap()),
            )),
            "then" => self.get_next_term(),
            "else" => self.get_next_term(),
            "0" => Ok(T::Zero),
            "succ" => Ok(T::Succ(Box::new(self.get_next_term().unwrap()))),
            "pred" => Ok(T::Pred(Box::new(self.get_next_term().unwrap()))),
            "iszero" => Ok(T::Iszero(Box::new(self.get_next_term().unwrap()))),
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
