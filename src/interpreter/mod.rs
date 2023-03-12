mod tree_walker;
mod value;

use tree_walker::Interpreter;

pub fn new() -> Interpreter {
    Interpreter::new()
}
