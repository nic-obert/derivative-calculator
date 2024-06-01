use std::rc::Rc;
use std::ptr;
use std::mem;
use std::fmt;

use crate::tokenizer::{SourceToken, Token, TokenValue};
use crate::errors;
use crate::ast::{FunctionTree, OpNode, OpValue};


pub type Priority = u16;


#[derive(Default)]
pub enum ParsingNodeValue<'a> {

    Parsed (OpNode<'a>),
    Unparsed { token: Token<'a>, priority: Priority },
    
    #[default]
    Placeholder
}

impl<'a> ParsingNodeValue<'a> {

    pub fn get_source(&self) -> &'a SourceToken {
        match self {
            ParsingNodeValue::Parsed(opnode) => &opnode.source,
            ParsingNodeValue::Unparsed { token, priority: _ } => &token.source,

            ParsingNodeValue::Placeholder => unreachable!()
        }
    }

}

impl fmt::Display for ParsingNodeValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsingNodeValue::Parsed (node) => write!(f, "{:?}", node.value),
            ParsingNodeValue::Unparsed { token, priority } => write!(f, "{} (Priority: {})", token.value, priority),
            _ => unreachable!()
        }
    }
}


pub struct ParsingNode<'a> {
    
    pub value: ParsingNodeValue<'a>,
    
    pub next: *mut ParsingNode<'a>,
    pub prev: *mut ParsingNode<'a>,

}


pub struct UnparsedTree<'a> {

    first_ptr: *mut ParsingNode<'a>,
    last_ptr: *mut ParsingNode<'a>,
    // Keeping the source here is a questionable choice, but it's simple and it works
    source: &'a str,

}

impl<'a> UnparsedTree<'a> {

    pub fn new(source: &'a str) -> Self {
        Self {
            first_ptr: ptr::null_mut(),
            last_ptr: ptr::null_mut(),
            source
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


    /// Extracts the node from the linked list, assuming it is in the list
    fn extract_node(&mut self, node_ptr: *mut ParsingNode<'a>) -> ParsingNodeValue<'a> {
        unsafe {
            let node = &mut *node_ptr;

            if let Some(prev) = node.prev.as_mut() {
                prev.next = node.next;
            } else {
                // The node to extract is the first node of the list, the only one with a prev nullptr
                self.first_ptr = node.next;
            }

            if let Some(next) = node.next.as_mut() {
                next.prev = node.prev;
            } else {
                // Same as above, last node is being extracted
                self.last_ptr = node.prev;
            }

            let value = mem::take(&mut node.value);

            ptr::drop_in_place(node);

            value            
        }   
    }


    /// Build a parsed tree representation of the function
    pub fn parse(mut self) -> FunctionTree<'a> {

        if self.first_ptr.is_null() {
            errors::invalid_input("Cannot build the AST of an empty function.");
        }

        // The linked list is now guaranteed not to be empty, there's no reason to worry about null pointers

        while let Some(node) = get_highest_priority(self.first_ptr) {

            // Assume the node hasn't been parsed yet. If it had been parsed, the `get_highest_priority` function should not have returned it
            let token = if let ParsingNodeValue::Unparsed { token, priority: _ } = &node.value { token } else { unreachable!() };


            macro_rules! extract_right {
                (parsed) => {{
                    if node.next.is_null() {
                        errors::parsing_error(&token.source, self.source, "Expected an operand to the right, but none was found");
                    }

                    match self.extract_node(node.next) {

                        ParsingNodeValue::Parsed(opnode) => opnode,

                        ParsingNodeValue::Unparsed { token, priority: _ }
                            => errors::parsing_error(&token.source, &self.source, "Invalid syntax, this token was not expected."),

                        ParsingNodeValue::Placeholder => unreachable!(),
                    }
                }};

                (unparsed) => {{
                    if node.next.is_null() {
                        errors::parsing_error(&token.source, self.source, "Expected an operand to the right, but none was found");
                    }

                    match self.extract_node(node.next) {

                        ParsingNodeValue::Parsed(opnode)
                            => errors::parsing_error(&opnode.source, &self.source, "Invalid syntax, this token was not expected."),

                        ParsingNodeValue::Unparsed { token, priority: _ }
                            => token,

                        ParsingNodeValue::Placeholder => unreachable!(),
                    }
                }};
            }


            macro_rules! extract_left {
                (parsed) => {{
                    if node.prev.is_null() {
                        errors::parsing_error(&token.source, self.source, "Expected an operand to the left, but none was found");
                    }

                    match self.extract_node(node.prev) {

                        ParsingNodeValue::Parsed(opnode) => opnode,

                        ParsingNodeValue::Unparsed { token, priority: _ }
                            => errors::parsing_error(&token.source, &self.source, "Invalid syntax, this token was not expected."),

                        ParsingNodeValue::Placeholder => unreachable!(),
                    }
                }};
            }

            macro_rules! parse_binary {
                ($op: ident) => {{

                    let left = Rc::new(extract_left!(parsed));
                    let right = Rc::new(extract_right!(parsed));                    

                    ParsingNodeValue::Parsed(OpNode {
                        source: Rc::clone(&token.source),
                        value: OpValue::$op { left, right },
                    })              
                }};
            }

            node.value = match token.value {

                // Binary operators
                TokenValue::Plus => parse_binary!(Add),
                TokenValue::Minus => parse_binary!(Sub),
                TokenValue::Mul => parse_binary!(Mul),
                TokenValue::Div => parse_binary!(Div),
                TokenValue::Pow => parse_binary!(Pow),

                TokenValue::ParenOpen => {
                    
                    let content = extract_right!(parsed);

                    let closing_paren = extract_right!(unparsed);
                    if !matches!(closing_paren.value, TokenValue::ParenClose) {
                        errors::parsing_error(&closing_paren.source, self.source, "Expected a closing parenthesis.");
                    }
                    
                    ParsingNodeValue::Parsed(OpNode {
                        source: Rc::clone(&token.source),
                        value: content.value, // Drop the parentheses, the tree structure will keep track of the operator hierarchy
                    })
                },

                TokenValue::Identifier(name) => {

                    // TODO: Can either be a function or a variable

                    ParsingNodeValue::Parsed(OpNode {
                        source: Rc::clone(&token.source),
                        value: OpValue::Variable(name)
                    })
                },
                
                TokenValue::Number(n)
                 => ParsingNodeValue::Parsed (OpNode {
                        source: Rc::clone(&token.source),
                        value: OpValue::Number(n)
                    }),

                TokenValue::Function(func) => {

                    let arg = extract_right!(parsed);
                    
                    ParsingNodeValue::Parsed(OpNode {
                        source: Rc::clone(&token.source),
                        value: OpValue::Function { 
                            func,
                            arg: Rc::new(arg)
                        }
                    })
                },
                    
                TokenValue::ParenClose 
                    => unreachable!(),
            };

        }

        // Convert the parsed tree into a proper ast

        // Assume the pointer is not null because the linked list should never be empty
        let root = unsafe { Box::from_raw(self.first_ptr) };

        // If the pointers are different, there is more than one root node in the tree
        if self.first_ptr != self.last_ptr {
            errors::parsing_error(root.value.get_source(), self.source, "Expression does not evaluate to a single value");
        }

        let root = if let ParsingNodeValue::Parsed(opnode) = root.value {
            Rc::new(opnode)
        } else {
            unreachable!()
        };

        FunctionTree {
            root
        }
    }

}

impl Drop for UnparsedTree<'_> {
    fn drop(&mut self) {
        // This function should normally do nothing since the tree should have been consumed in its entirety when building.
        // Here Drop is implemented both for the sake of completeness and in case the programmer didn't build the tree.
        
        let mut node_ptr = self.first_ptr;

        while let Some(node) = unsafe { node_ptr.as_ref() } {

            node_ptr = node.next;

            unsafe { 
                ptr::drop_in_place(node as *const ParsingNode as *mut ParsingNodeValue);
            }
        }
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
            node_ptr = node.next;
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

