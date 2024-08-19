use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    process,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    if argc > 2 {
        println!("Usage: lox[ script]");
        process::exit(-1);
    } else if argc == 2 {
        run_file(&args[1])
    } else {
        run_repl();
    }
}

fn run_file(path: &str) {
    let mut file = File::open(&path).unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    println!("{}", content);
}

fn run_repl() {
    let mut input = String::new();

    loop {
        input.clear();

        print!(">>> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        println!("{}", input);
    }
}
