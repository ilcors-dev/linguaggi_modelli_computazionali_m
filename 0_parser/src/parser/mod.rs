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

    pub fn parse_exp(&mut self) -> bool {
        let mut t1: bool = self.parse_term();
        println!("t1: {}", t1);
        while let Some(token) = &self.current_token {
            if token.get_token() == '+' {
                self.current_token = self.scanner.get_next_token();
                let t2: bool = self.parse_term();
                t1 = t1 && t2;
            } else if token.get_token() == '-' {
                self.current_token = self.scanner.get_next_token();
                let t2: bool = self.parse_term();
                t1 = t1 && t2;
            } else {
                return t1;
            }
        }
        t1
    }

    pub fn parse_term(&mut self) -> bool {
        let mut f1: bool = self.parse_factor();

        while let Some(token) = &self.current_token {
            if token.get_token() == '*' {
                self.current_token = self.scanner.get_next_token();
                let f2: bool = self.parse_factor();
                f1 = f1 && f2;
            } else if token.get_token() == ':' {
                self.current_token = self.scanner.get_next_token();
                let f2: bool = self.parse_factor();
                f1 = f1 && f2;
            } else {
                return f1;
            }
        }
        f1
    }

    pub fn parse_factor(&mut self) -> bool {
        if self.current_token.is_none() {
            return false;
        }

        if self.current_token.as_ref().unwrap().get_token() == '(' {
            self.current_token = self.scanner.get_next_token();
            let inner_exp: bool = self.parse_exp();
            if self.current_token.as_ref().unwrap().get_token() == ')' {
                self.current_token = self.scanner.get_next_token();
                return inner_exp;
            } else {
                return false;
            }
        } else if self.current_token.as_ref().unwrap().is_number() {
            self.current_token = self.scanner.get_next_token();
            return true;
        } else {
            return false;
        }
    }

    // public boolean parseTerm() {
}
