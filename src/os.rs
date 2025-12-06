#[allow(unused_imports)]
use std::process::Command;
#[cfg(target_os = "macos")]
pub fn get_os() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("system_profiler SPSoftwareDataType | grep \"System Version:\" | awk -F' ' '{print $3, $4}'")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
#[cfg(target_os = "freebsd")]
pub fn get_os() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("uname")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
#[cfg(target_os = "openbsd")]
pub fn get_os() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("uname -r")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
#[cfg(target_os = "linux")]
pub fn get_os() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("hostnamectl | grep \"Operating System\" | cut -d: -f2")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
#[cfg(target_os = "illumos")]
pub fn get_os() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("uname")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
