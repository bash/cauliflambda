#![feature(box_patterns)]

mod parsers;
pub mod syntax;
pub use parsers::*;
mod lowering;
pub use lowering::*;
mod diagnostics;
pub mod namefree;
pub use diagnostics::*;
mod reducing;
pub use reducing::*;
pub mod evaluation;
