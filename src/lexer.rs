use std::iter::Peekable;
use std::str::Chars;


#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Hai, Kthxbye, Obtw, Tldr, Maek, Oic, Gimmeh, Mkay, Head, Title, 
    Paragraf, Bold, Italics, List, Item, Newline,
    Soundz(String), Vidz(String), IHaz, ItIz, LemmeSee, I,
    VarDef(String), VarVal(String), Text(String),
}


pub trait LexicalAnalyzer {
    fn get_char(&mut self) -> Option<char>;
    fn add_char(&mut self, c: char);
    fn lookup(&self, s: &str) -> bool;
}

/// primary lexer struct, does conversion of strings to tokens
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    pub line: usize,
    current_lexeme: String,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            input: source.chars().peekable(),
            line: 1,
            current_lexeme: String::new(),
        }
    }

    ///main function for creating tokens with input, character by character
    /// also looks for # and general text
    pub fn lex(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(&c) = self.peek_char() {
            if c.is_whitespace() {
                self.get_char();
                continue;
            }

            // for words with # 
            if c == '#' {
                self.get_char(); 

                let annotation_core = self.consume_annotation_core(); 
                let ann_upper = annotation_core.to_uppercase();

                
                match ann_upper.as_str() { 
                    "HAI" => tokens.push(Token::Hai),
                    "KTHXBYE" => tokens.push(Token::Kthxbye),
                    "OBTW" => tokens.push(Token::Obtw),
                    "TLDR" => tokens.push(Token::Tldr),
                    "MAEK" => tokens.push(Token::Maek),
                    "OIC" => tokens.push(Token::Oic),
                    "MKAY" => tokens.push(Token::Mkay),

                    "GIMMEH" => { //structure for words with #
                        tokens.push(Token::Gimmeh);
                        let next_word = self.consume_word().to_uppercase(); //consumes GIMMEH command
                        match next_word.as_str() {
                            "BOLD" | "ITALICS" | "TITLE" => {
                                self.skip_whitespace(); //Consumes text,
                                let text = self.consume_text();
                                //pushes bold/italic/title tokens
                                match next_word.as_str() {
                                    "BOLD" => tokens.push(Token::Bold),
                                    "ITALICS" => tokens.push(Token::Italics),
                                    "TITLE" => tokens.push(Token::Title),
                                    _ => {}
                                }
                                if !text.is_empty() {
                                    tokens.push(Token::Text(text)); //pushed seperate so parser can look for closing tag
                                }
                            }// Rest of Gimmeh portions, follow similar format
                            "SOUNDZ" | "VIDZ" => {
                                self.skip_whitespace();
                                let address = self.consume_word();
                                match next_word.as_str() {
                                    "SOUNDZ" => tokens.push(Token::Soundz(address)),
                                    "VIDZ" => tokens.push(Token::Vidz(address)),
                                    _ => {}
                                }
                            }
                            "ITEM" => tokens.push(Token::Item),
                            "NEWLINE" => tokens.push(Token::Newline),
                            _ => return Err(format!("Unknown GIMMEH command '{}' on line {}", next_word, self.line)),
                        }
                    }
                    //For I Haz 
                    "I" => { 
                        if self.consume_word().to_uppercase() == "HAZ" {
                            tokens.push(Token::IHaz);
                            // self.skip_whitespace();
                            // let var_name = self.consume_word();
                            // tokens.push(Token::VarDef(var_name)); when uncommented the test at bottom work but messes up actual test cases
                        } else {
                            return Err(format!("Expected 'HAZ' after '#I' on line {}", self.line));
                        }
                    } 
                    "IT" => { //if IZ doesnt follow then returns error
                        if self.consume_word().to_uppercase() == "IZ" {
                            tokens.push(Token::ItIz);
                            // self.skip_whitespace();
                            // let var_value = self.consume_word();
                            // tokens.push(Token::VarVal(var_value)); same as above, messes up test cases but fixes bottom ones
                        } else {
                            return Err(format!("Expected 'IZ' after '#IT' on line {}", self.line));
                        }
                    }
                    "LEMME" => { 
                        if self.consume_word().to_uppercase() == "SEE" {
                            tokens.push(Token::LemmeSee);
                        } else {
                            return Err(format!("Expected 'SEE' after '#LEMME' on line {}", self.line));
                        }
                    }

                    "HEAD" => tokens.push(Token::Head),
                    "PARAGRAF" => tokens.push(Token::Paragraf),
                    "LIST" => tokens.push(Token::List),

                    _ => return Err(format!("Unknown annotation '#{}' on line {}", annotation_core, self.line)),
                }
            } else { //text without #, should fix ambiguity
                let word = self.consume_word();
                let upper_word = word.to_uppercase();

                match upper_word.as_str() { 
                    //Tokens that dont have a # that need to be found
                    "PARAGRAF" => tokens.push(Token::Paragraf),
                    "LIST" => tokens.push(Token::List),
                    "HEAD" => tokens.push(Token::Head),

                   
                    _ => {
                        //text for paragraph conent
                        let mut text = word; 
                        text.push_str(&self.consume_text());
                        
                        if !text.is_empty() {
                            tokens.push(Token::Text(text));
                        }
                    }
                }
            }
        } 
        Ok(tokens)
    }

// Helper methods below
   //looks at next char without consuming
    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

   
    fn consume_annotation_core(&mut self) -> String {
        let mut s = String::new();
        while let Some(&c) = self.peek_char() {
            if c.is_whitespace() || c == '#' { 
                break;
            }
            s.push(c);
            self.get_char();
        }
        s
    }
//consumes characters that form a single word and stops at whitespace or # 
    fn consume_word(&mut self) -> String {
        let mut s = String::new();

       
        self.skip_whitespace();

       
        while let Some(&c) = self.peek_char() {
            if c.is_whitespace() || c == '#' {
                break;
            }
            s.push(c);
            self.get_char();
        }
        s
    }

    //consumes all characters until it reaches a #
    fn consume_text(&mut self) -> String {
        let mut s = String::new();
        while let Some(&c) = self.peek_char() {
            if c == '#' {
                break; 
            }
            s.push(c);
            self.get_char();
        }
        s
    }
//skips whitespace 
    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if c.is_whitespace() {
                self.get_char();
            } else {
                break;
            }
        }
    }
}


impl<'a> LexicalAnalyzer for Lexer<'a> {
    fn get_char(&mut self) -> Option<char> {
        let c = self.input.next();
        if c == Some('\n') {
            self.line += 1;
        }
        c
    }

    fn add_char(&mut self, c: char) {
        self.current_lexeme.push(c);
    }

    fn lookup(&self, s: &str) -> bool {
        matches!(
            s.to_uppercase().as_str(),
            "#HAI" | "#KTHXBYE" | "#OBTW" | "#TLDR" | "#MAEK" | "#OIC" | "#MKAY"
                | "#GIMMEH" | "#I" | "#IT" | "#LEMME"
                | "#HEAD" | "#PARAGRAF" | "#LIST"
        )
    }
}
//assert_eq tests, didnt know if we had to write them ourselves or not
#[cfg(test)]
mod tests {
    use super::*; 

   

    
    #[test]
    fn test_trait_basic_program() {
        let source = "#HAI\n#KTHXBYE";
        let mut lexer = Lexer::new(source);

        let actual_tokens = lexer.lex().unwrap();
        
        let expected_tokens = vec![
            Token::Hai,
            Token::Kthxbye,
        ];

        
        assert_eq!(actual_tokens, expected_tokens); 
    }

    
    #[test]
    fn test_trait_variable_definition() {
        let source = "#I HAZ myvar #IT IZ 42 #MKAY";
        let mut lexer = Lexer::new(source);

        let actual_tokens = lexer.lex().unwrap();
        
        let expected_tokens = vec![
            Token::IHaz,
            Token::VarDef("myvar".to_string()),
            Token::ItIz,
            Token::VarVal("42".to_string()),
            Token::Mkay,
        ];

        assert_eq!(actual_tokens, expected_tokens); 
    }

   #[test]
    fn test_complex_inline_text() {
        let source = "#MAEK PARAGRAF Sample text #OIC";
        let mut lexer = Lexer::new(source);

        let actual_tokens = lexer.lex().unwrap();
        
        let expected_tokens = vec![
            Token::Maek,
            Token::Paragraf,
            Token::Text("Sample text ".to_string()),
            Token::Oic,
        ];

        assert_eq!(actual_tokens, expected_tokens); 
    }
}
