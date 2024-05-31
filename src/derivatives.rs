use std::rc::Rc;

use crate::functions::Functions;
use crate::ast::{FunctionTree, OpNode, OpValue};


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

macro_rules! number {
    ($source_node:ident, $num:literal) => {
        op_node!($source_node, OpValue::Number($num as f64))
    }
}


fn derive_node<'a>(node: &OpNode<'a>, dvar: &'a str) -> Rc<OpNode<'a>> {

    match &node.value {

        OpValue::Number(_)
        // f(x) = n
        // f'(x) = 0
         => number!(node, 0),

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
                            right: number!(node, 2) // 2
                        })                    
            }),

        OpValue::Variable(name) => {
        // f(x) = x
        // f'(x) = 1
        // or
        // f(x) = any
        // f'(x) = 0

            if *name == dvar {
                number!(node, 1)
            } else {
                number!(node, 0)
            }
        },
        
        OpValue::Function { func, arg }
        // f(x) = a(b(x))
        // f'(x) = a'(b(x)) * b'(x)
        => op_node!(node,
            OpValue::Mul {
                left: derive_function(*func, Rc::clone(arg), dvar), // a'(b(x))
                right: derive_node(arg, dvar) // b'(x)
            }
        ),
        
        OpValue::Pow { left, right } => {
        // f(x) = a(x) ^ b
        // f'(x) = b * a(x) ^ (b - 1) * a'(x)
        // or
        // f(x) = a(x) ^ b(x)
        // f'(x) = e ^ ( b(x) * ln(a(x)) ) * ( b'(x) * ln(a(x)) + b(x) * a'(x)/a(x) )

            let bx = derive_node(&right, dvar);

            // If the derivative of a function is zero, it's not a function of the derivation variable
            if matches!(bx.value, OpValue::Number(0.0)) {
                // The exponent is constant, so treat this operation as a regular power
                op_node!(node,
                    OpValue::Mul {
                        left: op_node!(node, // (b * a(x)) ^ (b - 1)
                            OpValue::Pow {
                                left: op_node!(node, // b(x) * a(x)
                                    OpValue::Mul {
                                        left: Rc::clone(right), // b
                                        right: Rc::clone(left) // a(x)
                                    }
                                ),
                                right: op_node!(node, // b - 1
                                    OpValue::Sub {
                                        left: Rc::clone(right), // b
                                        right: number!(node, 1) // 1
                                    }
                                )
                        }), 
                        right: derive_node(left, dvar) // a'(x)
                    }
                )
            } else {
                // The exponent is a function of the derivation variable, treat this opearation as an exponentiation
                op_node!(node,
                    OpValue::Mul {
                        left: op_node!(node, // e ^ (b(x) * ln(a(x)))
                            OpValue::Pow {
                                left: op_node!(node, OpValue::Variable("e")), // e
                                right: op_node!(node, // b(x) * ln(a(x))
                                    OpValue::Mul {
                                        left: Rc::clone(right), // b(x)
                                        right: op_node!(node, // ln( a(x) )
                                            OpValue::Function {
                                                func: Functions::NaturalLog,
                                                arg: Rc::clone(left)
                                            }
                                        ),
                                    }
                                ),
                            }
                        ),
                        right: op_node!(node, // (b'(x) * ln(a(x))) + (b(x) * a'(x)/a(x))
                            OpValue::Add {
                                left: op_node!(node, // b'(x) * ln(a(x))
                                    OpValue::Mul {
                                        left: derive_node(right, dvar), // b'(x)
                                        right: op_node!(node, // ln( a(x) )
                                            OpValue::Function {
                                                func: Functions::NaturalLog,
                                                arg: Rc::clone(left)
                                            }
                                        )
                                    }
                                ),
                                right: op_node!(node, // b(x) * a'(x)/a(x)
                                    OpValue::Mul {
                                        left: Rc::clone(right), // b(x)
                                        right: op_node!(node, // a'(x) / a(x)
                                            OpValue::Div {
                                                left: derive_node(left, dvar), // a'(x)
                                                right: Rc::clone(left)
                                            }
                                        )
                                    }
                                ),
                            }
                        ),
                    }
                )
            }

        },

    }

}


fn derive_function<'a>(func: Functions, arg: Rc<OpNode<'a>>, dvar: &'a str) -> Rc<OpNode<'a>> {
    match func {

        Functions::Sin
        // f(x) = sin(a(x))
        // f'(x) = cos(a(x)) * a'(x)
         => op_node!(arg, 
            OpValue::Mul {
                left: op_node!(arg, // cos(a(x))
                    OpValue::Function { 
                        func: Functions::Cos, 
                        arg: Rc::clone(&arg)
                    }
                ),
                right: derive_node(&arg, dvar) // a'(x)
            }
        ),

        Functions::Cos
        // f(x) = cos(a(x))
        // f'(x) = -sin(a(x)) * a'(x)
         => op_node!(arg,
            OpValue::Mul {
                left: op_node!(arg, // -sin(a(x))
                    OpValue::Mul {
                        left: number!(arg, -1), // -1
                        right: op_node!(arg,
                            OpValue::Function {
                                func: Functions::Sin,
                                arg: Rc::clone(&arg)
                            })
                    }),
                right: derive_node(&arg, dvar) // a'(x)
            }
        ),

        Functions::Tan
        // f(x) = tan(a(x))
        // f'(x) = sec(a(x))^2 * a'(x)
         => op_node!(arg,
            OpValue::Mul {
                left: op_node!(arg, // sec(a(x)) ^ 2
                    OpValue::Pow {
                        left: op_node!(arg, // sec( a(x) )
                            OpValue::Function {
                                func: Functions::Secant,
                                arg: Rc::clone(&arg)
                            }
                        ),
                        right: number!(arg, 2) // ^2
                    }
                ),
                right: derive_node(&arg, dvar) // a'(x)

            }
        ),

        Functions::Arcsin
        // f(x) = asin(a(x))
        // f'(x) = a'(x) / sqrt(1 - a(x)^2)
         => op_node!(arg,
            OpValue::Div {
                left: derive_node(&arg, dvar), // a'(x)
                right: op_node!(arg, // sqrt(1 - a(x)^2)
                    OpValue::Function {
                        func: Functions::SquareRoot,
                        arg: op_node!(arg, // 1 - a(x)^2
                            OpValue::Sub {
                                left: number!(arg, 1), // 1
                                right: op_node!(arg, // a(x)^2
                                    OpValue::Pow {
                                        left: Rc::clone(&arg), // a(x)
                                        right: number!(arg, 2) // ^2
                                    }
                                )
                            }
                        )
                    }
                )
            }
        ),

        Functions::Arccos
        // f(x) = asin(a(x))
        // f'(x) = - a'(x) / sqrt(1 - a(x)^2)
         => op_node!(arg,
            OpValue::Mul {
                left: number!(arg, -1), // -1
                right: op_node!(arg, // a'(x) / sqrt(1 - a(x)^2)
                    OpValue::Div {
                        left: derive_node(&arg, dvar), // a'(x)
                        right: op_node!(arg, // sqrt(1 - a(x)^2)
                            OpValue::Function {
                                func: Functions::SquareRoot,
                                arg: op_node!(arg, // 1 - a(x)^2
                                    OpValue::Sub {
                                        left: number!(arg, 1), // 1
                                        right: op_node!(arg, // a(x)^2
                                            OpValue::Pow {
                                                left: Rc::clone(&arg), // a(x)
                                                right: number!(arg, 2) // ^2
                                            }
                                        )
                                    }
                                )
                            }
                        )
                    }
                ),
            }
        ),

        Functions::Arctan
        // f(x) = tan(a(x))
        // f'(x) = a'(x) / (1 + a(x)^2)
         => op_node!(arg,
            OpValue::Div {
                left: derive_node(&arg, dvar), // a'(x)
                right: op_node!(arg, // 1 + a(x)^2
                    OpValue::Add {
                        left: number!(arg, 1), // 1
                        right: op_node!(arg, // a(x) ^ 2
                            OpValue::Pow {
                                left: Rc::clone(&arg), // a(x)
                                right: number!(arg, 2) // ^2
                            }
                        )
                    }
                )
            }
        ),

        Functions::SquareRoot => todo!(),
        Functions::NaturalLog => todo!(),
        Functions::Secant => todo!(),
        
        
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

