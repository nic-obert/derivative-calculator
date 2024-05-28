use std::fmt;
use std::ptr;
use std::rc::Rc;

use crate::errors;
use crate::tokenizer::SourceToken;
use crate::tokenizer::{Token, TokenValue};


pub type Priority = u16;


pub enum ParsingNodeValue<'a> {
    Parsed (OpNode<'a>),
    Unparsed { token: Token<'a>, priority: Priority }
}

impl fmt::Display for ParsingNodeValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsingNodeValue::Parsed (node) => todo!(),
            ParsingNodeValue::Unparsed { token, priority } => write!(f, "{} (Priority: {})", token.value, priority),
        }
    }
}


pub struct ParsingNode<'a> {
    
    pub value: ParsingNodeValue<'a>,
    
    pub next: *mut ParsingNode<'a>,
    pub prev: *mut ParsingNode<'a>,

}


pub enum OpValue {
    
    Number(f64)

}


pub struct OpNode<'a> {
    pub source: Rc<SourceToken<'a>>,
    pub value: OpValue
}


pub struct FunctionTree<'a> {

    root: *mut OpNode<'a>,

}

// impl fmt::Display for FunctionTree<'_> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let mut node_ptr = self.root;

//         while let Some(node) = unsafe { node_ptr.as_ref() } {

//             writeln!(f, "{}", node.value)?;

//             node_ptr = node.next;
//         }

//         Ok(())
//     }
// }


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


    /// Build a parsed tree representation of the function
    pub fn build(self) -> FunctionTree<'a> {

        if self.first_ptr.is_null() {
            errors::invalid_input("Cannot build the AST of an empty function.");
        }

        // The linked list is now guaranteed not to be empty, there's no reason to worry about null pointers

        loop {

            let node = if let Some(node) = get_highest_priority(self.first_ptr) {
                node
            } else {
                break;
            };

            // Assume the node hasn't been parsed yet. If it had been parsed, the `get_highest_priority` function should not have returned it
            let token = if let ParsingNodeValue::Unparsed { token, priority: _ } = &node.value { token } else { unreachable!() };

            match token.value {

                TokenValue::Plus => todo!(),
                TokenValue::Minus => todo!(),
                TokenValue::Mul => todo!(),
                TokenValue::Div => todo!(),
                TokenValue::Pow => todo!(),
                TokenValue::ParenOpen => todo!(),
                TokenValue::Identifier(_) => todo!(),
                
                TokenValue::Number(n) => {
                    node.value = ParsingNodeValue::Parsed (OpNode {
                        source: token.source.clone(),
                        value: OpValue::Number(n)
                    })
                },
                
                TokenValue::ParenClose 
                    => unreachable!(),
            }

        }

        todo!()        
    }

}


/// Assumes the passed pointer is not null
fn get_highest_priority(nodes: *const ParsingNode) -> Option<&mut ParsingNode> {

    let mut highest_priority = None;
    
    let mut node_ptr = nodes;

    while let Some(node) = unsafe { node_ptr.as_ref() } {

        let node_priority = if let ParsingNodeValue::Unparsed { token: _, priority } = node.value {
            priority
        } else {
            // The node has already been parsed, it has no priority
            continue;
        };

        // TODO: This branching could be avoided by moving the else branch outside the loop
        // This is fairly ok, though, because this program does not have to be extremely efficient
        if let Some((_hp_node, hp)) = highest_priority {
            if node_priority > hp {
                highest_priority = Some((node_ptr, node_priority));
            }
        } else {
            highest_priority = Some((node_ptr, node_priority));
        }

        node_ptr = node.next;
    }

    highest_priority.map(|(node, _)| 
        unsafe { &mut *(node as *mut ParsingNode) }
    )
}

