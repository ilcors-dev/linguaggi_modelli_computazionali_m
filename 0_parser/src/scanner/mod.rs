// class MyScanner extends java.util.StringTokenizer {
//     public MyScanner(String txt){
//     super(txt);
//     }
//     public Token getNextToken(){
//     try{
//     return new Token(nextToken().trim());
//     }
//     catch (java.util.NoSuchElementException e){
//     return null;
//     }
//     }
//     }

use crate::token::Token;

pub struct Scanner {
    txt: String,
    index: usize,
}

impl Scanner {
    pub fn new(txt: String) -> Scanner {
        Scanner { txt, index: 0 }
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        let c = self.txt.chars().nth(self.index);

        self.index += 1;

        if let Some(c) = c {
            Some(Token::new(c))
        } else {
            None
        }
    }
}
