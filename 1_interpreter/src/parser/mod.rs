use crate::{scanner::Scanner, token::Token};

pub struct Parser {
    scanner: Scanner,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(mut scanner: Scanner) -> Parser {
        let current_token = scanner.get_next_token();

        Parser {
            scanner,
            current_token,
        }
    }

    pub fn parse_exp(&mut self) -> usize {
        let mut t1: usize = self.parse_term();

        while let Some(token) = &self.current_token {
            if token.get_token() == '+' {
                self.current_token = self.scanner.get_next_token();
                let t2: usize = self.parse_term();
                t1 = t1 + t2
            } else if token.get_token() == '-' {
                self.current_token = self.scanner.get_next_token();
                let t2: usize = self.parse_term();
                t1 = t1 - t2;
            } else {
                return t1;
            }
        }

        println!("t1: {}", t1);
        t1
    }

    pub fn parse_term(&mut self) -> usize {
        let mut f1: usize = self.parse_factor();

        while let Some(token) = &self.current_token {
            if token.get_token() == '*' {
                self.current_token = self.scanner.get_next_token();
                let f2: usize = self.parse_factor();
                f1 = f1 * f2;
            } else if token.get_token() == ':' {
                self.current_token = self.scanner.get_next_token();
                let f2: usize = self.parse_factor();
                f1 = f1 / f2;
            } else {
                return f1;
            }
        }

        println!("f1: {}", f1);

        f1
    }

    pub fn parse_factor(&mut self) -> usize {
        if self.current_token.is_none() {
            panic!("Missing token")
        }

        println!("current_token: {:?}", self.current_token);
        if self.current_token.as_ref().unwrap().get_token() == '(' {
            self.current_token = self.scanner.get_next_token();
            let inner_exp: usize = self.parse_exp();
            if self.current_token.as_ref().unwrap().get_token() == ')' {
                self.current_token = self.scanner.get_next_token();
                return inner_exp;
            } else {
                panic!("Missing closing parenthesis")
            }
        } else if self.current_token.as_ref().unwrap().is_number() {
            let tk = self.current_token.as_ref().unwrap().get_as_usize();
            self.current_token = self.scanner.get_next_token();
            return tk;
        } else {
            panic!("NaN")
        }
    }
}
