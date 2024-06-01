use clap:: Parser;
use::anyhow::{Context, Result};
use serde_json::{Value};

#[derive(Parser)]
struct Cli {
    command: String,
    path: std::path::PathBuf,
}

// example cargo run -- ".[] | select(.name == \"foo\")" users.json

// users.json
// [
//     {
//         "name": "foo",
//         "age": 20
//     },
//     {
//         "name": "bar",
//         "age": 30
//     }
// ]

// output: 
// {
//         "name": "foo",
//         "age": 20
//  },
fn main() ->Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    println!("Command: {}", args.command);
    println!("Path: {}", args.path.display());
    // parse command and get .[] , select(.name == "foo")
    let command = jq::parse_command(&args.command);
    
    if let Some(filter) = &command.filter_operation {
        let key_value: Vec<&str> = filter.split("==").collect();
        let key = key_value[0].replace(".", "").replace(" ", "");
        let value = key_value[1].replace(" ", "");
        println!("key: {}", key);
        println!("value: {}", value);
        // read file
        let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
        // parse json
        let json: serde_json::Value = serde_json::from_str(&content)?;
        let user: &Value = jq::match_user(key, &value ,&json)?;
        // pretty print
        // like
        // {
        //     "name": "foo",
        //     "age": 20
        // }
        println!("{}", serde_json::to_string_pretty(user)?);
    } else {
        let err = anyhow::anyhow!("PARSE COMMAND FAILED");
        eprintln!("ERROR: {}", err);
        std::process::exit(1);
    }
    Ok(())
}

