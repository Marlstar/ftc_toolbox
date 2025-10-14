use std::path::PathBuf;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
pub struct AdbVersion(usize, usize, usize);
impl std::fmt::Display for AdbVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}.{}.{}", self.0, self.1, self.2))
    }
}

pub async fn installed_version() -> Result<AdbInstall, AdbError> {
    let stdout = super::command()
        .arg("--version")
        .output().await
        .map_err(|_| AdbError::NotInstalled)?
        .stdout;
    let adb_version = String::from_utf8(stdout).unwrap();

    tracing::debug!("Got valid stdout from adb");

    const SEMVER_REGEX: &str = "(?<v1>([0-9]+))\\.(?<v2>([0-9]+))\\.(?<v3>([0-9]+))";
    let server_version_reg = Regex::new(&format!("Android Debug Bridge version {SEMVER_REGEX}")).unwrap();
    let version_reg = Regex::new(&format!("Version {SEMVER_REGEX}")).unwrap();
    let path_reg = Regex::new("Installed as (?<path>(.*))").unwrap();

    let capture = server_version_reg.captures(&adb_version)
        .ok_or(AdbError::NotInstalled)?;
    let server_version = AdbVersion(
        capture["v1"].parse::<usize>().unwrap(),
        capture["v2"].parse::<usize>().unwrap(),
        capture["v3"].parse::<usize>().unwrap()
    );

    tracing::debug!("adb server version: {server_version}");

    let capture = version_reg.captures(&adb_version)
        .unwrap();
        // .ok_or(AdbError::NotInstalled)?;
    let version = AdbVersion(
        capture["v1"].parse::<usize>().unwrap(),
        capture["v2"].parse::<usize>().unwrap(),
        capture["v3"].parse::<usize>().unwrap()
    );

    tracing::debug!("adb version: {version}");

    let capture = path_reg.captures(&adb_version)
        .ok_or(AdbError::NotInstalled)?;
    let path = PathBuf::from(&capture["path"]);

    tracing::debug!("adb path: {}", path.display());

    Ok(AdbInstall { version, server_version, path, is_local: is_installed_locally() })
}

pub fn is_installed_locally() -> bool {
    super::install::BINPATH.exists()
}

#[derive(Debug, Clone)]
pub struct AdbInstall {
    version: AdbVersion,
    server_version: AdbVersion,
    path: PathBuf,
    is_local: bool,
}

#[derive(Debug)]
pub enum AdbError {
    NotInstalled,
}
