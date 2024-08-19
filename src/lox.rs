use crate::scanner::Scanner;

use std::{
    fs::File,
    io::{self, Read, Write},
};

pub fn run_file(path: &str) {
    let mut file = File::open(&path).unwrap();

    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();

    run_code(&code);
}

pub fn run_repl() {
    let mut code = String::new();

    loop {
        code.clear();

        print!(">>> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut code).unwrap();
        let input = code.trim();

        run_code(&input);
    }
}

fn run_code(code: &str) {
    let mut scanner = Scanner::new(code);
    let tokens = scanner.scan_tokens();

    println!("{:?}", tokens)
}
