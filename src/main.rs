use std::env;

use piec::Compiler;

// TODO: Provide polished CLI via clap
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let compiler = Compiler::from_file(file_path).unwrap();
    compiler.compile().unwrap();
}
