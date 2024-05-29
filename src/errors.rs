use colored::Colorize;

use crate::tokenizer::SourceToken;


pub fn print_source_context(source: &str, char_pointer: usize) {

    println!("{}", source);
    println!("{:>char_pointer$}{}", "", "^".bright_red().bold());

}


pub fn invalid_token(token: &SourceToken, source: &str, message: &str) -> ! {

    println!("Invalid token `{}` at column {}:\n", token.string, token.column);

    print_source_context(source, token.column);

    println!("\n{}\n", message);

    std::process::exit(1);
}


pub fn invalid_input(message: &str) -> ! {

    println!("Invalid input:\n{}\n", message);

    std::process::exit(1);
}


pub fn parsing_error(token: &SourceToken, source: &str, message: &str) -> ! {

    println!("Parsing error on token `{}` at column {}:\n", token.string, token.column);

    print_source_context(source, token.column);

    println!("\n{}\n", message);

    std::process::exit(1);
}

