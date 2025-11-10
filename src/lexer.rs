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

    
    pub fn lex(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(&c) = self.peek_char() {
            if c.is_whitespace() {
                self.get_char();
                continue;
            }

            
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

                    "GIMMEH" => {
                        tokens.push(Token::Gimmeh);
                        let next_word = self.consume_word().to_uppercase();
                        match next_word.as_str() {
                            "BOLD" | "ITALICS" | "TITLE" => {
                                self.skip_whitespace();
                                let text = self.consume_text();

                                match next_word.as_str() {
                                    "BOLD" => tokens.push(Token::Bold),
                                    "ITALICS" => tokens.push(Token::Italics),
                                    "TITLE" => tokens.push(Token::Title),
                                    _ => {}
                                }
                                if !text.is_empty() {
                                    tokens.push(Token::Text(text));
                                }
                            }
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

                    "I" => { 
                        if self.consume_word().to_uppercase() == "HAZ" {
                            tokens.push(Token::IHaz);
                        } else {
                            return Err(format!("Expected 'HAZ' after '#I' on line {}", self.line));
                        }
                    }
                    "IT" => { 
                        if self.consume_word().to_uppercase() == "IZ" {
                            tokens.push(Token::ItIz);
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
            } else { 
                let word = self.consume_word();
                let upper_word = word.to_uppercase();

                match upper_word.as_str() {
                    
                    "PARAGRAF" => tokens.push(Token::Paragraf),
                    "LIST" => tokens.push(Token::List),
                    "HEAD" => tokens.push(Token::Head),

                   
                    _ => {
                        
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

