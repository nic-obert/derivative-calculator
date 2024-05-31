use std::fmt;


macro_rules! declare_functions {
    ($($name:ident $repr:ident),+) => {
        
/// Known mathematical functions
#[derive(Clone, Copy)]
pub enum Functions {

    $($name),+

}

impl Functions {

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            $(stringify!($repr) => Some(Self::$name),)+
            _ => None
        }
    }

}

impl fmt::Display for Functions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            $(Self::$name => write!(f, stringify!($repr)),)+
        }
    }
}

    };
}

declare_functions! {

    Sin sin,
    Cos cos,
    Tan tan,
    Arcsin asin,
    Arccos acos,
    Arctan atan,
    SquareRoot sqrt,
    NaturalLog ln,
    Secant sec

}

