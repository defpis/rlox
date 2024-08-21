use rlox::lox;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    // Check exit code by `echo $?`
    if argc > 2 {
        println!("Usage: lox[ script]");
        process::exit(-1); // 255
    } else if argc == 2 {
        lox::run_file(&args[1]) // 0
    } else {
        lox::run_repl(); // 130
    }
}
