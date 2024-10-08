use crate::{interpreter::interpret, parser::parse, scanner::scan_tokens};
use std::{
    fs::File,
    io::{self, Read, Write},
};

fn skip_out(code: &str) -> &str {
    let mut tmp: Vec<&str> = code.split("------ output ------").collect();
    tmp = tmp[0].split("------ error ------").collect();
    tmp[0]
}

pub fn run_file(path: &str) {
    let mut file = File::open(&path).unwrap();

    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();

    run_code(skip_out(&code));
}

pub fn run_repl() {
    let mut code = String::new();

    loop {
        code.clear();

        print!(">>> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut code).unwrap();
        let input = code.trim();

        run_code(input);
    }
}

pub fn run_code(code: &str) {
    let tokens = scan_tokens(code);
    // println!("{:?}", tokens);

    let statements = parse(tokens);
    // println!("{:?}", statements);

    interpret(&statements);
}
