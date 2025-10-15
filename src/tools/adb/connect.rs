use std::string::FromUtf8Error;
use tokio::time::{Duration, timeout};
use regex::Regex;

const TIMEOUT: u64 = 3000;

pub async fn connect() -> Result<(), ConnectError> {
    async fn stdout() -> Result<Vec<u8>, ConnectError> {
        Ok(super::command()
            .arg("connect")
            .arg("192.168.43.1")
            .output().await?
            .stdout)
    }

    tracing::debug!("Attempting connection");
    let stdout = timeout(Duration::from_millis(TIMEOUT), stdout()).await
        .map_err(|_| ConnectError::Timeout)??;
    let stdout = String::from_utf8(stdout)?;

    let failed_reg = Regex::new("failed to connect to '(?<address>([0-9\\.:]+))': (?<reason>(.*))").unwrap();
    if let Some(captures) = failed_reg.captures(&stdout) {
        return Err(ConnectError::Connection(captures["reason"].to_string()));
    }
    tracing::info!("Connected successfully");
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
    #[error("Connection timed out")]
    Timeout,
}
