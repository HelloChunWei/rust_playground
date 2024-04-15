pub fn match_user<'a>(key: String, name: &str, data: &'a serde_json::Value) -> Result<&'a serde_json::Value, anyhow::Error> {
    for data in data.as_array().unwrap() {
        if data[key.clone()] == name {
            return Ok(data);
        }
    }
    Err(anyhow::anyhow!("not found"))
}
