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

pub fn force() -> Result<(), InstallError> {
    tracing::info!("Installing ADB to {}", BINDIR.display());

    let client = reqwest::blocking::Client::new();
    let response = client.execute(client.get(URL).build()?)?;

    tracing::info!("Downloaded ADB zip");

    let mut archive = zip::ZipArchive::new(Cursor::new(response.bytes()?))?;
    archive.extract_unwrapped_root_dir(&*BINDIR, |path| path.starts_with("platform-tools"))?;

    Ok(())
}

pub fn if_necessary() -> Result<AdbInstall, InstallError> {
    if let Ok(ver) = check() {
        tracing::info!("Found existing adb installation");
        return Ok(ver);
    }

    force()?;
    Ok(check().expect("adb installation botched"))
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
