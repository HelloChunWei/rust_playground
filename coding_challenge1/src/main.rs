use clap::Parser;

#[derive(Parser)]
struct Cli {
    command: String,
}

fn main() {
    let args = Cli::parse();
    println!("Command: {}", args.command);
    if args.command != "c" {
        println!("Command not found");
        std::process::exit(1);
    }
    println!("Hello, world!");
    let content = std::fs::read_to_string("./test.txt").unwrap();
    // calculate the btye
    // 1 byte = 8 bits
    // 1 letter = 1 byte
    let byte_length = content.len();
    println!("Byte length: {}", byte_length);

    // calculate the line length
    let line_length = content.lines().count();
    println!("Line length: {}", line_length);

    let word_length = content.split_whitespace().count();
    println!("Word length: {}", word_length);
}
