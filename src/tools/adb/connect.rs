use std::string::FromUtf8Error;

use regex::Regex;

pub async fn connect() -> Result<(), ConnectError> {
    let stdout = super::command()
        .arg("connect")
        .arg("192.168.43.1")
        .output().await?
        .stdout;
    let stdout = String::from_utf8(stdout)?;

    let failed_reg = Regex::new("failed to connect to '(?<address>([0-9\\.:]+))': (?<reason>(.*))").unwrap();
    if let Some(captures) = failed_reg.captures(&stdout) {
        return Err(ConnectError::Connection(captures["reason"].to_string()));
    }
    Ok(())
}

#[derive(Debug, thiserror::Error, from_variants::FromVariants)]
pub enum ConnectError {
    #[error("IO error: {0:?}")]
    IO(std::io::Error),
    #[error("Malformed command output: {0:?}")]
    MalformedCommandOutput(FromUtf8Error),
    #[error("ADB connection error: {0}")]
    Connection(String),
}
