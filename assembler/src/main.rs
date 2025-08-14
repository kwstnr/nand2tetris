use std::env;
use std::path::Path;
use std::fs;

mod tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filepath>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];
    let path = Path::new(file_path);
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_string();
    let file_content = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let lines = file_content.lines().collect::<Vec<&str>>();

    let tokenizedLines = tokenizer::tokenize(&lines);
}
