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
    // 解析 command 取出 .[] 和 select(.name == "foo")
    // 只能用純字串處理？
    // 型別有點小麻煩
    let mut command = args.command.split("|");

    // get fisrt command like: .[] replace " and space
    let first_command = command.next().unwrap().replace("\"","").replace(" ", "");
    // get second command like: select(.name == "foo") replace " and space
    let second_command = command.next().unwrap().replace("\"","").replace(" ", "");
    // if first_commmand is not ".[]", throw error
    if first_command != ".[]" {
        let err = anyhow::anyhow!("first command must be .[]");
        eprintln!("ERROR: {}", err);
        std::process::exit(1);
    }
    // if second_command is not match pattern "select()", throw error
    if !second_command.starts_with("select(") || !second_command.ends_with(")") {
        let err = anyhow::anyhow!("second command must be select()");
        eprintln!("ERROR: {}", err);
        std::process::exit(1);
    }
    // find key and value in select()
    let key_value = second_command.replace("select(", "").replace(")", "");
    let key_value: Vec<&str> = key_value.split("==").collect();
    let key = key_value[0].replace(".", "");
    let value = key_value[1];
    println!("key: {}", key);
    println!("value: {}", value);

    // read file
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    // parse json
    let json: serde_json::Value = serde_json::from_str(&content)?;
    let user: &Value = match_user(key, &value ,&json)?;
    // pretty print
    // like
    // {
    //     "name": "foo",
    //     "age": 20
    // }
    println!("{}", serde_json::to_string_pretty(user)?);


    Ok(())
}

fn match_user<'a>(key: String, name: &str, data: &'a serde_json::Value) -> Result<&'a serde_json::Value, anyhow::Error> {
    for data in data.as_array().unwrap() {
        if data[key.clone()] == name {
            return Ok(data);
        }
    }
    Err(anyhow::anyhow!("not found"))
}