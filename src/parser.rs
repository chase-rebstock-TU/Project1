use crate::lexer::Token;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::process::Command;


pub trait SyntaxAnalyzer {
    fn parse_lolcode(&mut self) -> Result<(), String>;
    fn parse_head(&mut self) -> Result<(), String>;
    fn parse_title(&mut self) -> Result<(), String>;
    fn parse_comment(&mut self) -> Result<(), String>;
    fn parse_body(&mut self) -> Result<(), String>;
    fn parse_paragraph(&mut self) -> Result<(), String>;
    fn parse_inner_paragraph(&mut self) -> Result<(), String>;
    fn parse_inner_text(&mut self) -> Result<(), String>;
    fn parse_variable_define(&mut self) -> Result<(), String>;
    fn parse_variable_use(&mut self) -> Result<(), String>;
    fn parse_bold(&mut self) -> Result<(), String>;
    fn parse_italics(&mut self) -> Result<(), String>;
    fn parse_list(&mut self) -> Result<(), String>;
    fn parse_list_items(&mut self) -> Result<(), String>;
    fn parse_inner_list(&mut self) -> Result<(), String>;
    fn parse_audio(&mut self) -> Result<(), String>;
    fn parse_video(&mut self) -> Result<(), String>;
    fn parse_newline(&mut self) -> Result<(), String>;
}

pub struct Parser {
    tokens: Vec<Token>,
    pos:usize,
    variables: HashMap<String, String>,
    output: String,
}

impl Parser {
    pub fn new (tokens: Vec<Token>) -> Self {
        Parser { 
            tokens, 
            pos: 0 ,
        variables: HashMap::new(),
        output: String::new(),
    }
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
                self.advance();
                Ok(())
            }else {
                Err(format!("Syntax Error: Expected {:?}, found {:?}", expected, token))
            }
        } else {
            Err(format!("Syntax Error: Unexpected end of input. Expected {:?}", expected))
        }
    }

    pub fn compile_and_run(&mut self, input_filename: &str) -> Result <(), String> {\
        self.parse()?;

        let output_filename = input_filename.strip_suffix(".lolmd")
        .unwrap_or(input_filename)
        .strip_suffix(".lol")
        .unwrap_or(input_filename)
        .to_owned() + ".html";

    let mut file = match File::create(&output_filename){
        Ok(f) => f,
        Err(e) => return Err(format!("Code Generation Error: Could not create output file '{}': {}", output_filename, e)),   
    };
    if let Err(e) = file.write_all(self.output.as_bytes()){
        return Err(format!("Code Generation Error: Could not write to output file: {}", e));

    }
let browser_result = Command::new("open") // mac only
            .arg(&output_filename)
            .spawn();

        match browser_result {
            Ok(_) => println!("Successfully compiled to '{}' and launched browser.", output_filename),
            Err(e) => println!("Warning: Could not launch web browser. Please open '{}' manually. Error: {}", output_filename, e),
        }
        Ok(())
    }

    pub fn parse(&mut self) -> Result<(), String> {
        self.output.push_str("<!DOCTYPE html>\n<html>\n");
        self.expect(&Token::Hai)?;
        self.parse_comments()?;
        self.parse_head()?;
        self.output.push_str("<body>\n");
        self.parse_body()?;
        self.output.push_str("</body>\n");
        self.expect(&Token::Kthxbye)?;
        self.output.push_str("</html>\n");

        if self.peek().is_some() {
            return Err(format!("Syntax Error: Content found after #KTHXBYE: {:?}", self.peek()));
        }

        Ok(())
    }
    fn parse_comments(&mut self) -> Result<(), String> {
        while self.peek() == Some(&Token::Obtw)  {
            self.advance();
            let mut found_tldr = false;
            while let Some(token) = self.peek() {
                if token == &Token:: Tldr {
                    self.advance();
                    found_tldr = true;
                    break;
                }
                self.advance();
            }
            if !found_tldr {
                return Err("Syntax Error: Comment started with #OBTW but never closed with #TLDR.".to_string());
            }
    }
    Ok(())
}
fn parse_head(&mut self) -> Result<(), String> {
    if self.peek() == Some(&Token::Maek){
        self.advance();
        self.expect(&Token::Head)?;
        self.output.push_str("<head>\n");
        self.parse_comments()?;
        self.parse_title()?;
        self.parse_comments()?;
        self.expect(&Token::Oic)?;
        self.output.push_str("</head>\n");
    }
    Ok(())
}


fn parse_title(&mut self) -> Result<(), String> {
    self.expect(&Token::Gimmeh)?;
    self.expect(&Token::Title)?;

    self.output.push_str("<title>");

    match self.peek(){
        Some(Token::Text(title)) => {
            self.output.push_str(title);
            self.advance()
        },
        _ => return Err ("Syntax Error: #GIMMEH TITLE must be followed by text.".to_string()),
    }
    self.expect(&Token::Mkay)?;
    self.output.push_str("</title>\n");
    Ok(())
}


fn parse_body(&mut self) -> Result<(), String> {
    while self.peek() != Some(&Token::Kthxbye){

        self.parse_comments()?;

        match self.peek() {
            Some(Token::Maek) => {
                self.advance();

                match self.peek() {
                    Some(Token::Paragraf) => self.parse_paragraph()?,
                    Some(Token::List) => self.parse_list()?,
                    _ => return Err(format!("Syntax Error: Expected PARAGRAF or LIST after #MAEK, found {:?}", self.peek())),
                }
            },
            Some(Token::IHaz) => self.parse_variable_define()?,
            Some(Token::LemmeSee) => self.parse_variable_use()?,
            Some(Token::Gimmeh) => self.parse_gimmeh_body_element()?,
            Some(Token::Text(_)) => self.parse_inner_text()?,

           
            Some(Token::Bold) | Some(Token::Italics) | Some(Token::Soundz(_)) | Some(Token::Vidz(_)) | Some(Token::Newline) => self.advance(),
            _ => {
                return Err(format!("Syntax Error: Unexpected token in document body: {:?}", self.peek()));
            }
        }
    }
    Ok(())
}


fn parse_paragraph(&mut self) -> Result<(), String> {
    self.expect(&Token::Paragraf)?;
    self.output.push_str("<p>");

    if self.peek() == Some(&Token::I) {
        self.advance();
        self.expect(&Token::IHaz)?;
        self.parse_variable_define_core()?;
    }
    
    while self.peek() != Some(&Token::Oic){
        self.parse_inner_paragraph()?;
    }
    self.expect(&Token::Oic)?;
    self.output.push_str("</p>\n");
    Ok(())
}



fn parse_inner_paragraph(&mut self) -> Result<(), String> {
    self.parse_comments()?;

    match self.peek() {
        Some(Token::Gimmeh) => {
            self.advance();
            match self.peek() {
               Some(Token::Bold) => self.parse_bold()?,
                Some(Token::Italics) => self.parse_italics()?,
                Some(Token::Soundz(_)) => self.parse_audio()?, 
                Some(Token::Vidz(_)) => self.parse_video()?,   
                Some(Token::Newline) => self.parse_newline()?,
                _ => return Err(format!("Syntax Error: Unknown #GIMMEH element inside PARAGRAF: {:?}", self.peek())),
            }
        },
        Some(Token::Maek)=> {
            self.advance();
            match self.peek() {
                Some(Token::List) => self.parse_list()?,
                _ => return Err(format!("Syntax Error: Expected LIST after #MAEK inside PARAGRAF, found {:?}",self.peek()))
            }
        },
        Some(Token::I) => {
            return Err("Syntax Error: Variable definition #I HAZ... not allowed mid-paragraph.".to_string());
        }
        Some(Token::LemmeSee) => self.parse_variable_use()?,
        Some(Token::Text(_)) => self.parse_inner_text()?,
        _ => return Err (format!("Syntax Error: Unexpected token inside paragraph: {:?}", self.peek())),
    }
    Ok(())
}
fn parse_variable_define(&mut self) -> Result<(), String> {
    self.expect(&Token::IHaz)?;
    self.parse_variable_define_core()
}

fn parse_variable_define_core(&mut self) -> Result<(), String> {
    let var_name  = match self.peek() {
        Some(Token::Text(name)) => {
        let name_string = name.clone();
        self.advance();
        name_string
    }
        _ => return Err("Syntax Error: Expected variable name (Text) after #I HAZ.".to_string())
};
    self.expect(&Token::ItIz)?;
    
    let var_value = match self.peek(){
        Some(Token::Text(value)) => {
            let value_string = value.clone();
            self.advance();
            value_string
        }
        _ => return Err("Syntax Error: Expected variable value (Text) after #IT IZ.".to_string()),
    };
    self.expect(&Token::Mkay)?;

    if self.variables.contains_key(&var_name){
        return Err(format!("Semantic Error: Variable '{}' is already defined.", var_name));
    }
    self.variables.insert(var_name, var_value);
    Ok(())
}
fn parse_variable_use(&mut self) -> Result<(), String> {
    self.expect(&Token::LemmeSee)?;

    let var_name = match self.peek(){
        Some(Token::Text(name)) => {
            let name_string = name.clone();
            self.advance();
            name_string
        }
        _ => return Err ("Syntax Error: Expected variable name (Text) after #LEMME SEE.".to_string()),
    }; 
    self.expect(&Token::Mkay)?;

    if let Some(value) = self.variables.get(&var_name) {
        self.output.push_str(value);
    } else {
        return Err(format!("Semantic Error: Variable '{}' used but not defined.", var_name));
    }
    Ok(())
}

fn parse_gimmeh_body_element(&mut self) -> Result<(), String> {
    self.expect(&Token::Gimmeh)?;
    match self.peek() {
        Some(Token::Bold) => self.parse_bold(),
        Some(Token::Italics) => self.parse_italics(),
        Some(Token::Soundz(_)) => self.parse_audio(),
        Some(Token::Vidz(_)) => self.parse_video(),
        Some(Token::Newline) => self.parse_newline(),
        _ => Err (format!("Syntax Error: Unexpected GIMMEH command in body: {:?}", self.peek())),
    }
 }


fn parse_audio(&mut self) -> Result<(), String> {
    let mut audio_src = String::new();
    match self.peek(){
        Some(Token::Soundz(src)) =>{
            audio_src = src.clone();
            self.advance();
        },
        _ => return Err ("Internal Error: parse audio called without Soundz token.".to_string()),
    }
    self.expect(&Token::Mkay)?;
    self.output.push_str(&format!("<audio controls><source src=\"{}\" type=\"audio/mp3\"></audio>\n", audio_src));
    Ok(())
}
fn parse_newline(&mut self) -> Result<(), String> {
    self.expect(&Token::Newline)?;
    self.output.push_str("<br>\n");
    Ok(())
}

fn parse_video(&mut self) -> Result<(), String> {
    let mut video_src = String::new();
        match self.peek() {
            Some(Token::Vidz(src)) => {
                video_src = src.clone();
                self.advance();
            },
            _ => return Err("Internal Error: parse_video called without Vidz token.".to_string()),
        }
        self.expect(&Token::Mkay)?;
        self.output.push_str(&format!("<iframe src=\"{}\" frameborder=\"0\" allowfullscreen></iframe>\n", video_src));
        Ok(())
    }
    fn parse_inner_text(&mut self) -> Result<(), String> {
        match self.peek() {
            Some(Token::Text(text)) => {
                self.output.push_str(text);
                self.advance()
            },
            _ => return Err("Internal Error: parse_inner_text called without Text token.".to_string())
        }
        Ok(())
    }

fn parse_bold(&mut self) -> Result<(), String> {
    self.expect(&Token::Bold)?;
    self.output.push_str("<b>");
    match self.peek() {
        Some(Token::Text(_)) => self.parse_inner_text()?,
        _ => {}
    }
    self.expect(&Token::Mkay)?;
    self.output.push_str("</b>");
    Ok(())
}
fn parse_italics(&mut self) -> Result<(), String> {
    self.expect(&Token::Italics)?;
    self.output.push_str("<i>");
    match self.peek() {
        Some(Token::Text(_)) => self.parse_inner_text()?,
        _ => {}
    }
    self.expect(&Token::Mkay)?;
    self.output.push_str("</i>");
    Ok(())
}
fn parse_list(&mut self) -> Result<(), String> {
    self.expect(&Token::List)?;
    self.output.push_str("<ul>\n");
    self.parse_list_items()?;
    self.expect(&Token::Oic)?;
    self.output.push_str("</ul>\n");
    Ok(())
}

fn parse_list_items(&mut self) -> Result<(), String> {
    let mut item_count = 0;
    self.parse_comments()?;
    while self.peek() == Some(&Token::Gimmeh) {
        self.advance();
        self.expect(&Token::Item)?;
        self.output.push_str("<li>");

        while self.peek() != Some(&Token::Mkay) && self.peek().is_some() {
            match self.peek() {
                Some(Token::Gimmeh) => {
                    self.advance();
                    match self.peek() {
                        Some(Token::Bold) => self.parse_bold()?,
                        Some(Token::Italics) => self.parse_italics()?,
                        _ => return Err (format!("Syntax Error: Only BOLD/ITALICS allowed in list item: {:?}", self.peek()))
                    }
                },
                Some(Token::Text(_)) => self.parse_inner_text()?,
                _ => break,
            }
        }
        self.expect(&Token::Mkay)?;
        self.output.push_str("</li>\n");
        item_count += 1;
        self.parse_comments()?;
    }
    if item_count == 0 {
        return Err("Syntax Error: #MAEK LIST must contain at least one #GIMMEH ITEM.".to_string());
    }
    Ok(())
}
fn parse_inner_list(&mut self) -> Result<(), String> {
    self.parse_list_items()
}


}
impl SyntaxAnalyzer for Parser {
    fn parse_lolcode(&mut self) -> Result<(), String> {
        self.parse()
    }

    fn parse_head(&mut self) -> Result<(), String> { self.parse_head() }
    fn parse_title(&mut self) -> Result<(), String> { self.parse_title() }
    fn parse_comment(&mut self) -> Result<(), String> { self.parse_comments() }
    fn parse_body(&mut self) -> Result<(), String> { self.parse_body() }
    fn parse_paragraph(&mut self) -> Result<(), String> { self.parse_paragraph() }
    fn parse_inner_paragraph(&mut self) -> Result<(), String> { self.parse_inner_paragraph() }
    fn parse_inner_text(&mut self) -> Result<(), String> { self.parse_inner_text() }
    fn parse_variable_define(&mut self) -> Result<(), String> { self.parse_variable_define() }
    fn parse_variable_use(&mut self) -> Result<(), String> { self.parse_variable_use() }
    fn parse_bold(&mut self) -> Result<(), String> { self.parse_bold() }
    fn parse_italics(&mut self) -> Result<(), String> { self.parse_italics() }
    fn parse_list(&mut self) -> Result<(), String> { self.parse_list() }
    fn parse_list_items(&mut self) -> Result<(), String> { self.parse_list_items() }
    fn parse_inner_list(&mut self) -> Result<(), String> { self.parse_inner_list() }
    fn parse_audio(&mut self) -> Result<(), String> { self.parse_audio() }
    fn parse_video(&mut self) -> Result<(), String> { self.parse_video() }
    fn parse_newline(&mut self) -> Result<(), String> { self.parse_newline() }
}

