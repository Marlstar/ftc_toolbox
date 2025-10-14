use std::string::FromUtf8Error;

pub async fn connect() -> Result<(), ConnectError> {
    let stdout = super::command()
        .arg("connect")
        .arg("192.168.43.1")
        .output().await?
        .stdout;
    let stdout = String::from_utf8(stdout)?;
    todo!()
}

#[derive(Debug, thiserror::Error, from_variants::FromVariants)]
pub enum ConnectError {
    #[error("IO error: {0:?}")]
    IO(std::io::Error),
    #[error("Malformed command output: {0:?}")]
    MalformedOutput(FromUtf8Error),
}
