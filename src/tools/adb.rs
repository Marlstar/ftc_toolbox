use std::process::Command;
use std::path::PathBuf;
use regex::Regex;

pub type AdbVersion = (usize, usize, usize);

pub fn adb_version() -> Result<AdbInstall, AdbError> {
    let stdout = Command::new("adb")
        .arg("--version")
        .output()
        .map_err(|_| AdbError::MalformedCommandOutput)?
        .stdout;
    let adb_version = String::from_utf8(stdout).unwrap();
    println!("stdout:\n{adb_version}");

    const SEMVER_REGEX: &str = "(?<v1>([0-9]+))\\.(?<v2>([0-9]+))\\.(?<v3>([0-9]+))";
    let server_version_reg = Regex::new(&format!("Android Debug Bridge version {SEMVER_REGEX}")).unwrap();
    let version_reg = Regex::new(&format!("Version {SEMVER_REGEX}-android-tools")).unwrap();
    let path_reg = Regex::new("Installed as (?<path>(.*))").unwrap();

    let capture = server_version_reg.captures(&adb_version)
        .ok_or(AdbError::NotInstalled)?;
    let server_version = (
        capture["v1"].parse::<usize>().unwrap(),
        capture["v2"].parse::<usize>().unwrap(),
        capture["v3"].parse::<usize>().unwrap()
    );

    let capture = version_reg.captures(&adb_version)
        .ok_or(AdbError::NotInstalled)?;
    let version = (
        capture["v1"].parse::<usize>().unwrap(),
        capture["v2"].parse::<usize>().unwrap(),
        capture["v3"].parse::<usize>().unwrap()
    );

    println!("path time");
    let capture = path_reg.captures(&adb_version)
        .ok_or(AdbError::NotInstalled)?;
    let path = PathBuf::from(&capture["path"]);

    Ok(AdbInstall { version, server_version, path })
}

#[derive(Debug, Clone)]
pub struct AdbInstall {
    version: AdbVersion,
    server_version: AdbVersion,
    path: PathBuf,
}

#[derive(Debug)]
pub enum AdbError {
    NotInstalled,
    MalformedCommandOutput,
}
