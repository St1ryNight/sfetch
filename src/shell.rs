use std::process::Command;

#[cfg(target_os = "macos")]
pub fn get_shell() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo $SHELL")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
#[cfg(target_os = "linux")]
pub fn get_shell() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo $SHELL")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
#[cfg(target_os = "freebsd")]
pub fn get_shell() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo $SHELL")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
#[cfg(target_os = "openbsd")]
pub fn get_shell() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo $SHELL")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
#[cfg(target_os = "illumos")]
pub fn get_shell() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo $SHELL")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
#[cfg(target_os = "windows")]
pub fn get_shell() -> String {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("echo powershell")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
