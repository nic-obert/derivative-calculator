use clap::Parser;


#[derive(Parser)]
pub struct CliParser {

    /// The input function to derive
    #[clap(required = true)]
    pub input_function: String,

    /// The derivation variable
    #[clap(short='d', default_value="x")]
    pub derivation_variable: String,

}

