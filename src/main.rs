mod evaluate;
mod parse;
mod scope;
mod token;
mod types;
mod repl;

pub use crate::evaluate::{define_builtins, evaluate};
pub use crate::parse::parse;
pub use crate::scope::Scope;
pub use crate::types::*;
use std::io;

fn main() -> io::Result<()> {
    crate::repl::repl()
}
