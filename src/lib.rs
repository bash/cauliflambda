#![feature(box_patterns)]

#[macro_export]
macro_rules! λ {
    ($($expr:tt)*) => {
        $crate::evaluation::Term::from($crate::parse_formula(stringify!($($expr)*)).unwrap().value)
    };
}

mod parsers;
pub mod syntax;
pub use parsers::*;
mod diagnostics;
pub use diagnostics::*;
mod default;
pub mod evaluation;
