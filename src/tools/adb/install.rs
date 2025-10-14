use std::path::PathBuf;
use std::sync::LazyLock;
use std::io::Cursor;
use super::check_install::AdbInstall;


#[cfg(windows)]
const URL: &str = "https://dl.google.com/android/repository/platform-tools-latest-windows.zip";
#[cfg(target_os = "macos")]
const URL: &str = "https://dl.google.com/android/repository/platform-tools-latest-darwin.zip";
#[cfg(target_os = "linux")]
const URL: &str = "https://dl.google.com/android/repository/platform-tools-latest-linux.zip";

pub static BINDIR: LazyLock<PathBuf> = LazyLock::new(|| directories::BaseDirs::new().unwrap().data_dir().join("ftc_toolbox/dep/adb"));
#[cfg(windows)]
pub static BINPATH: LazyLock<PathBuf> = LazyLock::new(|| BINDIR.join("adb.exe"));
#[cfg(not(windows))]
pub static BINPATH: LazyLock<PathBuf> = LazyLock::new(|| BINDIR.join("adb"));

pub use super::check_install::installed_version as check;

pub async fn force() -> Result<(), InstallError> {
    tracing::info!("Installing ADB to {}", BINDIR.display());

    let client = reqwest::Client::new();
    let response = client.execute(client.get(URL).build()?).await?;

    tracing::info!("Downloaded ADB zip");

    let mut archive = zip::ZipArchive::new(Cursor::new(response.bytes().await?))?;
    archive.extract_unwrapped_root_dir(&*BINDIR, |path| path.starts_with("platform-tools"))?;

    tracing::info!("Extracted ADB zip");
    tracing::info!("ADB installed successfully");

    Ok(())
}

pub async fn if_necessary() -> Result<AdbInstall, InstallError> {
    if let Ok(ver) = check().await {
        tracing::info!("Found existing adb installation");
        return Ok(ver);
    }

    force().await?;
    Ok(check().await.expect("adb installation botched"))
}

#[derive(Debug, thiserror::Error, from_variants::FromVariants)]
pub enum InstallError {
    #[error("Web request error: {0:?}")]
    Reqwest(reqwest::Error),
    #[error("Filesystem error: {0:?}")]
    Filesystem(std::io::Error),
    #[error("Zip archive error: {0:?}")]
    Archive(zip::result::ZipError),
}
