# JQ in rust practice

command `cargo run --  ".[] | select(.name == "John")" user.json`

在 JSON 檔案中尋找 name = John 的資料。
目前只先支援這個功能