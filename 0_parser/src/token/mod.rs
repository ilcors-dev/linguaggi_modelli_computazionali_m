// public class Token {
//     private String tk;
//     public Token(String tk){ this.tk = tk;}
//     public boolean isNumber() {
//     try{ Integer.parseInt(tk); }
//     catch(NumberFormatException e){ return false; }
//     return true;
//     }
//     public String toString(){ return tk; }
//     public boolean equals(Object o){
//     if (o instanceof String) {
//     return this.tk.equals((String)o);
//     }
//     else if (o instanceof Token) {
//     Token that = (Token)o;
//     return this.tk.equals(that.tk);
//     }
//     else return false;
//     }
//     }

pub struct Token {
    tk: char,
}

impl Token {
    pub fn new(tk: char) -> Token {
        Token { tk }
    }

    pub fn is_number(&self) -> bool {
        self.tk.is_digit(10)
    }

    pub fn get_token(&self) -> char {
        self.tk
    }
}
