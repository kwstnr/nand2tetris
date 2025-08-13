use std::env;
use assembler::run;

fn main() { println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    run(&args);
}
