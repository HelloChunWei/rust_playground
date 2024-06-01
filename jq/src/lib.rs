
pub struct Command {
    pub array_operation: bool,
    pub filter_operation: Option<String>,
}
pub fn match_user<'a>(key: String, name: &str, data: &'a serde_json::Value) -> Result<&'a serde_json::Value, anyhow::Error> {
    for data in data.as_array().unwrap() {
        if data[key.clone()] == name {
            return Ok(data);
        }
    }
    Err(anyhow::anyhow!("not found"))
}
pub fn parse_command(command_str: &str) -> Command {
    let  array_operation = false;
    let mut filter_operation = None;

    // if first_commmand is not ".[]", throw error
    if !command_str.starts_with(".[]") {
        let err = anyhow::anyhow!("first command must be .[]");
        eprintln!("ERROR: {}", err);
        std::process::exit(1);
    } 

    // 解析 filter 操作
    if let Some(filter_str) = command_str.strip_prefix(".[] | select(") {
        if filter_str.ends_with(")") {
            filter_operation = Some(filter_str[..filter_str.len() - 1].to_string());
        }
    } else {
        let err = anyhow::anyhow!("second command must be select()");
        eprintln!("ERROR: {}", err);
        std::process::exit(1);
    }

    Command {
        array_operation,
        filter_operation,
    }
}