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

    let function_tree = tokens.parse();

    println!("Original function tree:\n{:?}", function_tree);

    let derivative_tree = derivatives::derive(&function_tree, &args.derivation_variable);

    println!("Derived function tree:\n{:?}", derivative_tree);

    println!("\n\nLinear derivative function:\n{}", derivative_tree);

    let simplified_derivative = derivative_tree.simplify();

    println!("\nSimplified derivative function tree:\n{:?}", simplified_derivative);

    println!("\n\nLinear simplified derivative function:\n{}", simplified_derivative);

}

