use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

/// This function is only available on Linux.use which to get executable path
///  and return it as a PathBuf
pub(crate) fn find_executable_in_path(executable_name: &str) -> Option<PathBuf> {
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

fn launch_executable(executable_path: &PathBuf, args: Option<Vec<&str>>, env_vars: &HashMap<String, &str>) {
    {
        let mut command_args: Vec<&str> = vec![];
        if let Some(args) = args {
            command_args = args
        }
        let mut child = Command::new(executable_path)
            .envs(env_vars)
            .args(command_args)
            .spawn()
            .expect("Failed to execute command");


        let status = child.wait().expect("Failed to wait on child");
        println!("child status was: {}", status);
    }
}

pub fn launch_discord() {
    let executable_path = find_executable_in_path("discord").unwrap();
    let mut env_vars = HashMap::new();
    env_vars.insert("http_proxy".to_string(), "http://127.0.0.1:10809");
    env_vars.insert("https_proxy".to_string(), "http://127.0.0.1:10809");

    let arg = vec!["proxy-server", "http://127.0.0.1:10809"];
    launch_executable(&executable_path, Some(arg), &env_vars);
}
