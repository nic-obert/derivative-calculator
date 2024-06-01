use std::fmt;
use std::rc::Rc;

use crate::tokenizer::SourceToken;
use crate::functions::Functions;


pub enum OpValue<'a> {
    
    Number (f64),
    Add { left: Rc<OpNode<'a>>, right: Rc<OpNode<'a>> },
    Sub { left: Rc<OpNode<'a>>, right: Rc<OpNode<'a>> },
    Mul { left: Rc<OpNode<'a>>, right: Rc<OpNode<'a>> },
    Div { left: Rc<OpNode<'a>>, right: Rc<OpNode<'a>> },
    Pow { left: Rc<OpNode<'a>>, right: Rc<OpNode<'a>> },
    Variable (&'a str),

    // Here Box<[]> must be used because Rc does not include the size of the slice
    /// A one-argument math function
    Function { func: Functions, arg: Rc<OpNode<'a>> },

}

impl fmt::Display for OpValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpValue::Number(n) => write!(f, "{}", n),
            OpValue::Add { left, right } => write!(f, "({} + {})", left.value, right.value),
            OpValue::Sub { left, right } => write!(f, "({} - {})", left.value, right.value),
            OpValue::Mul { left, right } => write!(f, "({} * {})", left.value, right.value),
            OpValue::Div { left, right } => write!(f, "({} / {})", left.value, right.value),
            OpValue::Pow { left, right } => write!(f, "({} ^ {})", left.value, right.value),
            OpValue::Variable(name) => write!(f, "{}", name),
            OpValue::Function { func, arg } => write!(f, "{}({})", func, arg.value),
        }
    }
}

impl OpValue<'_> {

    pub fn fmt_indented(&self, mut indent: usize, f: &mut fmt::Formatter) -> fmt::Result {

        for _ in 0..indent {
            write!(f, "| ")?;
        }
        if indent != 0 {
            write!(f, "|_ ")?;
        }
        indent += 1;

        match self {
            OpValue::Number(n) => write!(f, "{}", n)?,
            OpValue::Add { left, right } => {
                writeln!(f, "+")?;
                left.value.fmt_indented(indent, f)?;
                writeln!(f)?;
                right.value.fmt_indented(indent, f)?;
            },
            OpValue::Sub { left, right } => {
                writeln!(f, "-")?;
                left.value.fmt_indented(indent, f)?;
                writeln!(f)?;
                right.value.fmt_indented(indent, f)?;
            },
            OpValue::Mul { left, right } => {
                writeln!(f, "*")?;
                left.value.fmt_indented(indent, f)?;
                writeln!(f)?;
                right.value.fmt_indented(indent, f)?;
            },
            OpValue::Div { left, right } => {
                writeln!(f, "/")?;
                left.value.fmt_indented(indent, f)?;
                writeln!(f)?;
                right.value.fmt_indented(indent, f)?;
            },
            OpValue::Pow { left, right } => {
                writeln!(f, "^")?;
                left.value.fmt_indented(indent, f)?;
                writeln!(f)?;
                right.value.fmt_indented(indent, f)?;
            },
            OpValue::Variable(name) => write!(f, "{}", name)?,
            OpValue::Function { func, arg } => {
                writeln!(f, "{}()", func)?;
                arg.value.fmt_indented(indent, f)?;
            },
        }

        Ok(())
    }

}

impl fmt::Debug for OpValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_indented(0, f)
    }
}


pub struct OpNode<'a> {
    pub source: Rc<SourceToken<'a>>,
    pub value: OpValue<'a>
}


pub struct FunctionTree<'a> {

    pub root: Rc<OpNode<'a>>,

}

impl fmt::Debug for FunctionTree<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.root.value)
    }
}

impl fmt::Display for FunctionTree<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root.value)
    }
}

