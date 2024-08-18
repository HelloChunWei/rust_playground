use clap::Parser;
use std::collections::HashMap;

#[derive(Parser)]
struct Cli {
    command: Option<String>,
    filename: Option<String>,
}

fn calculate_byte(content: String) {
    let byte_length = content.len();
    println!("Byte length: {}", byte_length);
}

fn calculate_line(content: String) {
    let line_length = content.lines().count();
    println!("Line length: {}", line_length);
}

fn calculate_word(content: String) {
    let word_length = content.split_whitespace().count();
    println!("Word length: {}", word_length);
}

fn calculate_char(content: String) {
    let char_length = content.chars().count();
    println!("Char length: {}", char_length);
}

fn main() {
    let mut command_map: HashMap<&str, fn(content: String)> = HashMap::new();
    command_map.insert("c", calculate_byte);
    command_map.insert("l", calculate_line);
    command_map.insert("w", calculate_word);
    command_map.insert("m", calculate_char);

    let args = Cli::parse();
    println!("commend: {:?}", args.command);

    let content = if let Some(filename) = args.filename {
        std::fs::read_to_string(filename)
    } else {
        std::fs::read_to_string("./test.txt")
    };

    if let Some(cmd) = &args.command {
        if let Some(command) = command_map.get(cmd.as_str()) {
            if let Ok(content) = content {
                command(content);
            } else {
                println!("can not read file");
                std::process::exit(1);
            }
        } else {
            println!("command not found");
            std::process::exit(1);
        }
    } else {
        println!("default command");
        match content {
            Ok(content) => {
                calculate_byte(content.clone());
                calculate_line(content.clone());
                calculate_word(content);
            }
            Err(_) => {
                println!("can not read file");
                std::process::exit(1);
            }
        }
    }
}
