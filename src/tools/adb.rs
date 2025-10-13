use std::process::Command;
use regex::Regex;

pub fn adb_version() -> Result<(usize, usize, usize), AdbError> {
    let stdout = Command::new("adb")
        .arg("--version")
        .output()
        .map_err(|_| AdbError::MalformedCommandOutput)?
        .stdout;
    let adb_version = String::from_utf8(stdout).unwrap();
    println!("stdout:\n{adb_version}");

    let r = Regex::new("Android Debug Bridge version (?<v1>([0-9]+))\\.(?<v2>([0-9]+))\\.(?<v3>([0-9]+))").unwrap();
    let capture = r.captures(&adb_version)
        .ok_or(AdbError::NotInstalled)?;
    let version = (
        capture["v1"].parse::<usize>().unwrap(),
        capture["v2"].parse::<usize>().unwrap(),
        capture["v3"].parse::<usize>().unwrap()
    );

    Ok(version)
}

#[derive(Debug)]
pub enum AdbError {
    NotInstalled,
    MalformedCommandOutput,
}
