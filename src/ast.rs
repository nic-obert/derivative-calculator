use std::fmt;
use std::ptr;

use crate::tokenizer::Token;


pub type Priority = u16;


pub enum ParsingNodeValue<'a> {
    Parsed {},
    Unparsed { token: Token<'a>, priority: Priority }
}

impl fmt::Display for ParsingNodeValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsingNodeValue::Parsed {} => todo!(),
            ParsingNodeValue::Unparsed { token, priority } => write!(f, "{} (Priority: {})", token.value, priority),
        }
    }
}


pub struct ParsingNode<'a> {
    
    pub value: ParsingNodeValue<'a>,
    
    pub next: *mut ParsingNode<'a>,
    pub prev: *mut ParsingNode<'a>,

}


pub struct AST<'a> {

    root: *mut ParsingNode<'a>,

}

impl fmt::Display for AST<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut node_ptr = self.root;

        while let Some(node) = unsafe { node_ptr.as_ref() } {

            writeln!(f, "{}", node.value)?;

            node_ptr = node.next;
        }

        Ok(())
    }
}


pub struct ASTBuilder<'a> {

    first_ptr: *mut ParsingNode<'a>,
    last_ptr: *mut ParsingNode<'a>,

}

impl<'a> ASTBuilder<'a> {

    pub fn new() -> Self {
        Self {
            first_ptr: ptr::null_mut(),
            last_ptr: ptr::null_mut(),
        }
    }


    pub fn push_token(&mut self, token: Token<'a>, positional_priority: Priority) {
        unsafe {

            let new_node = Box::leak(Box::new(
                ParsingNode {
                    value: ParsingNodeValue::Unparsed { 
                        priority: positional_priority + token.value.base_priority(),
                        token, 
                    },
                    next: ptr::null_mut(),
                    prev: self.last_ptr,
                }
            ));

            if let Some(last) = self.last_ptr.as_mut() {
                last.next = new_node;
            } else {
                // If last is null, first is also null
                self.first_ptr = new_node;
            }
            
            self.last_ptr = new_node;
        }
    }


    pub fn build(self) -> AST<'a> {
        AST { root: self.first_ptr }
    }

}

