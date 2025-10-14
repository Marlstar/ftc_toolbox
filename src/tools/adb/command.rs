use tokio::process::Command;
use super::check_install::is_installed_locally;

pub(super) fn command() -> Command {
    if is_installed_locally() {
        Command::new(&*super::install::BINPATH)
    } else { Command::new("adb") }
}
