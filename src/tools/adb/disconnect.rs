use std::string::FromUtf8Error;

pub async fn disconnect() -> Result<(), DisconnectError> {
    let stdout = super::command()
        .arg("disconnect")
        .output().await?
        .stdout;
    let stdout = String::from_utf8(stdout)?;

    if stdout.contains("disconnected everything") {
        Ok(())
    } else {
        Err(DisconnectError::Unknown)
    }
}

#[derive(Debug, thiserror::Error, from_variants::FromVariants)]
pub enum DisconnectError {
    #[error("IO error: {0:?}")]
    IO(std::io::Error),
    #[error("Malformed command output: {0:?}")]
    MalformedCommandOutput(FromUtf8Error),
    #[error("Unknown error")]
    Unknown,
}
