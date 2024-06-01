mod cli_parser;
mod tokenizer;
mod ast;
mod errors;
mod tests;
mod functions;
mod derivatives;
mod parsing_tree;

use clap::Parser;

use cli_parser::CliParser;


fn main() {
    
    let args = CliParser::parse();

    if !tokenizer::is_variable(&args.derivation_variable) {
        errors::invalid_input(format!("Derivation variable `{}` is not a valid variable name", args.derivation_variable).as_str());
    }

    let tokens = tokenizer::tokenize(&args.input_function);

    // println!("{}", tokens);

    let ast = tokens.parse();

    println!("Original function tree:\n{:?}", ast);

    let dast = derivatives::derive(&ast, &args.derivation_variable);

    println!("Derived function tree:\n{:?}", dast);

    println!("\n\nLinear derivative function:\n{}", dast);

}

