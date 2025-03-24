use std::env;
use std::process;
use std::fs;
use std::io::{self, Write};

mod lexer;
use lexer::*;

mod ast;
use ast::*;

mod parser;
use parser::*;

mod logging;
use logging::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_count = args.len() - 1;
    if arg_count > 1 {
        println!("Usage: lox/lox.exe [script]");
        process::exit(64);
    } else if arg_count == 1 {
        let temp_arg = args[1].clone();
        run_file(temp_arg);
    } else {
        run_prompt();
    }
}

fn run_file(file_path: String) {
    let copy = file_path.clone();
    match fs::read_to_string(file_path) {
        Ok(content) => run(content),
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            eprintln!("Provided path: {}", copy);
        }
    }
}

fn run_prompt() {
    println!("Starting Lox Prompt! :)");
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed

        input.clear(); // Clear the input buffer
        if stdin.read_line(&mut input).is_err() {
            break;
        }

        let trimmed: &str = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        run(trimmed.to_string());
    }
}

fn run(source: String) {
	let mut lexer : lexer::Lexer = Lexer::new(source);
	let tokens :&Vec<lexer::Token> = lexer.scan_tokens();
	let mut parser : parser::Parser = Parser::new(tokens.clone());
    let expression_res = parser.parse();
    match expression_res {
        Ok(expr) => {
            let mut printer = ast::ASTPrinter;
            println!("{}", printer.print(&expr));
        },
        Err(_) => {
            eprintln!("parser error!");
        }
    }
} 
