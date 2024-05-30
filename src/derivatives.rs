use std::rc::Rc;

use crate::{ast::{FunctionTree, OpNode, OpValue}, functions::Functions};


fn derive_node<'a>(node: &OpNode<'a>, dvar: &'a str) -> Rc<OpNode<'a>> {

    macro_rules! op_node {
        ($source_node:ident, $value:expr) => {
            Rc::new(
                OpNode {
                    source: Rc::clone(&$source_node.source),
                    value: $value
                }
            )
        };
    }

    match &node.value {

        OpValue::Number(_)
        // f(x) = 2
        // f'(x) = 0
         => op_node!(node,
                OpValue::Number(0.0)
            ),

        OpValue::Add { left, right }
        // f(x) = a(x) + b(x)
        // f'(x) = a'(x) + b'(x)
         => op_node!(node,
                OpValue::Add { 
                    left: derive_node(left, dvar), // a'(x)
                    right: derive_node(right, dvar) // b'(x)
                }
            ),

        OpValue::Sub { left, right }
        // f(x) = a(x) - b(x)
        // f'(x) = a'(x) - b'(x)
         => op_node!(node,
                OpValue::Sub { 
                    left: derive_node(left, dvar), // a'(x)
                    right: derive_node(right, dvar) // b'(x)
                }
            ),

        OpValue::Mul { left, right }
        // f(x) = a(x) * b(x)
        // f'(x) = a'(x) * b(x) + b'(x) * a(x)
         => op_node!(node,
                OpValue::Add {
                    left: op_node!(node, // a'(x) * b(x)
                        OpValue::Mul {
                            left: derive_node(left, dvar), // a'(x)
                            right: Rc::clone(right) // b(x)
                    }),
                    right: op_node!(node, // b'(x) * a(x)
                        OpValue::Mul {
                            left: Rc::clone(left), // a(x)
                            right: derive_node(right, dvar) // b'(x)
                    })
                }
            ),

        OpValue::Div { left, right }
        // f(x) = a(x) / b(x)
        // f'(x) = ( a'(x) * b(x) - a(x) * b'(x) ) / g(x)^2
         => op_node!(node,
                OpValue::Div {
                    left: op_node!(node, // (a'(x) * b(x)) - (a(x) * b'(x))
                        OpValue::Sub {
                            left: op_node!(node, // a'(x) * b(x)
                                OpValue::Mul {
                                    left: derive_node(left, dvar), // a'(x)
                                    right: Rc::clone(right) // b(x)
                            }),
                            right: op_node!(node, // a(x) * b'(x)
                                OpValue::Mul {
                                    left: Rc::clone(left), // a(x)
                                    right: derive_node(right, dvar) // b'(x)
                            }),
                    }),
                    right: op_node!(node, // g(x)^2
                        OpValue::Pow {
                            left: Rc::clone(right), // b(x)
                            right: op_node!(node, OpValue::Number(2.0)) // 2
                        })                    
            }),

        OpValue::Variable(name) => {
        // f(x) = x
        // f'(x) = 1
        // or
        // f(x) = k
        // f'(x) = 0

            if *name == dvar {
                op_node!(node, OpValue::Number(1.0))
            } else {
                op_node!(node, OpValue::Number(0.0))
            }
        },
        
        OpValue::Function { func, arg }
        // f(x) = a(b(x))
        // f'(x) = a'(b(x)) * b'(x)
        => op_node!(node,
            OpValue::Mul {
                left: derive_function(*func, arg), // a'(b(x))
                right: derive_node(arg, dvar) // b'(x)
            }
        ),
        
        OpValue::Pow { left, right }
        // f(x) = a(x) ^ b(x)
        // f'(x) = ( b(x) * a(x) ^ ( b(x) - 1 ) ) * a'(x)
         => op_node!(node,
            OpValue::Mul {
                left: op_node!(node, // (b(x) * a(x)) ^ (b(x) - 1)
                    OpValue::Pow {
                        left: op_node!(node, // b(x) * a(x)
                            OpValue::Mul {
                                left: Rc::clone(right), // b(x)
                                right: Rc::clone(left) // a(x)
                            }
                        ),
                        right: op_node!(node, // b(x) - 1
                            OpValue::Sub {
                                left: Rc::clone(right), // b(x)
                                right: op_node!(node, OpValue::Number(1.0)) // 1
                            }
                        )
                }), 
                right: derive_node(left, dvar) // a'(x)
            }
        ),

    }

}


fn derive_function<'a>(func: Functions, arg: &OpNode<'a>) -> Rc<OpNode<'a>> {
    match func {
        Functions::Sin => todo!(),
        Functions::Cos => todo!(),
        Functions::Tan => todo!(),
        Functions::Tanh => todo!(),
        Functions::Arcsin => todo!(),
        Functions::Arccos => todo!(),
        Functions::Arctan => todo!(),
        Functions::SquareRoot => todo!(),
    }
}


/// Derive `func` with respect to `dvar`.
pub fn derive<'a>(func: &FunctionTree<'a>, dvar: &'a str) -> FunctionTree<'a> {

    /*
        Perform a recursive depth-first traversal of the tree.
        At every node, calculate the derivative of the opnode.
        The derivative of each opnode is dependent on the node's children.
        Immutable references are used to allow reusing some nodes without having to
        re-calculate them.
    */

    FunctionTree {
        root: derive_node(&func.root, dvar)
    }

}

