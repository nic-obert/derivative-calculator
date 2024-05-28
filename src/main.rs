mod cli_parser;
mod tokenizer;
mod ast;
mod errors;

use clap::Parser;

use cli_parser::CliParser;


fn main() {
    
    let args = CliParser::parse();

    let tokens = tokenizer::tokenize(&args.input_function);

    if !tokenizer::is_variable(&args.derivation_variable) {
        todo!("error")
    }

    // println!("{}", tokens);

}

