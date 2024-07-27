use std::fs;
use std::env;
use serde::Deserialize;
use solana_program::msg;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Secrets {
    DbConn: String
}


pub fn getConnString() -> String
{
    let basePathOption = env::current_dir().expect("Couldn't find current dir");
    let secretsPath: String = format!("{}{}", basePathOption.into_os_string().into_string().expect("OS error converting a string"), "/secrets/secrets.json");

    let data = fs::read_to_string(secretsPath).unwrap();

    let secrets: Secrets = serde_json::from_str(&data).expect("JSON was not well-formatted");
    return secrets.DbConn;
}