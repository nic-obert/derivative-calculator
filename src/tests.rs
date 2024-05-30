
#[cfg(test)]
mod tests {
    use crate::tokenizer;


    #[test]
    fn test() {
        let foo = "1+1";
        let tokens = tokenizer::tokenize(foo);
        let _ast = tokens.parse();
    }

}

