#![feature(box_patterns)]
#![feature(default_free_fn)]

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
pub mod evaluation;
