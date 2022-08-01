mod asynceval;
// mod builtins;
// mod evaluate;
// mod exec;
mod parse;
// mod scope;
mod test;
mod token;
mod types;

// pub use crate::builtins::define_builtins;
// pub use crate::evaluate::evaluate;
pub use crate::parse::parse;
// pub use crate::scope::Scope;
pub use crate::types::*;
// use std::env::args;
use std::io;

fn main() -> io::Result<()> {
    // let mut args = args();
    // // Who cares about the first arg
    // args.next();
    // if let Some(filename) = args.next() {
    //     crate::exec::exec_file(&filename)
    // } else {
    //     crate::exec::repl()
    // }
    Ok(())
}
