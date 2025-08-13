pub fn run(args: &Vec<String>) {
    println!("{:#?}", args);

    let asm_file_path = args.get(1).expect("No file path provided");
}
