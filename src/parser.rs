use crate::tokens::Token;

pub struct Parser {
    tokens: Vec<token>,
    pos:usize;
}

impl Parser {
    pub fn new (tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }
    fn expect(&mut self, expected: &Token) -> Result <(), String> {
        if let Some(token) = self.peek() {
            if token == expected {
                Ok(())
            }else {
                Err(format!("Expected {:?}, found at {:?} at position {}", expected, token, self.pos))
            }
        } else {
            Err(format!("Unexpected end of input. Expected {:?}", expected))
        }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        self.expect(&Token::Hai)?;
        self.parse_comments()?;
        self.parse_head()?;
        self.parse_body()?;
        self.expect(&Token::Kthxbye)?;
        Ok(())
    }
    fn parse_comments(&mut self) -> Result<(), String> {
        while let Some(Token:Obtw) = self.peek() {
            self.advance();
            match self.peek() {
                Some(Token:: Text(_)) => self.advance(),
                _ => return Err ("Expected text after OBTW".to_string()),
            }
            self.expect(&Token::Tldr)?;
        }
        OK(())
    }
// fn parse_head()
//to be implemented

//fn parse_title()
// to be implemented

//fn parse_body
//to be implemented

//fn parse_paragraph
//to be implemented

//fn parse_inner_paragraph
//to be implemented

// still need to handle variables, inline formatting and list structure (task 3)

}