use std::env;
use std::fs;
use std::process;

 mod parser;
 mod lexer;
 use parser::Parser;
 use lexer::Lexer;

 fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2{
        eprintln!("Usage: lolcompiler <input_file.lol>");
        eprintln!("Error: Compiler requires exactly one command-line argument (the input file name).");
        process::exit(1);
    }

    let input_filename = &args[1];

    if !input_filename.ends_with(".lol"){
        eprintln!("Error: Input file must have the required '.lol' extension.");
        eprintln!("Received: {}", input_filename);
        process::exit(1);
    }
let source_code = match fs::read_to_string(input_filename) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("File Error: Could not read file '{}'. {}", input_filename, e);
            process::exit(1);
        }
    };
    let mut lexer = Lexer::new(&source_code);
    let tokens = match lexer.lex() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer Error: Tokenization failed:\n{}", e);
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);

    match parser.compile_and_run(input_filename) {
        Ok(_) => {

        },
        Err(e) => {
            eprintln!("Compilation failed: {}", e);
            process::exit(1);
        }
    }
 }
