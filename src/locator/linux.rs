use std::path::PathBuf;
use std::process::Command;

/// This function is only available on Linux.use which to get executable path
///  and return it as a PathBuf
pub fn find_executable_in_path(executable_name: &str) -> Option<PathBuf> {
    let output = Command::new("which")
        .arg(executable_name)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let path = String::from_utf8(output.stdout).expect("Not UTF-8");
        return Some(PathBuf::from(path.trim()));
    }

    None
}