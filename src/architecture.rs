use std::process::Command;
#[cfg(target_os = "macos")]
pub fn get_cputype() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("uname -m")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
#[cfg(target_os = "linux")]
pub fn get_cputype() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("uname -m")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
#[cfg(target_os = "freebsd")]
pub fn get_cputype() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("uname -p")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
#[cfg(target_os = "openbsd")]
pub fn get_cputype() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("uname -p")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
#[cfg(target_os = "illumos")]
pub fn get_cputype() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("uname -p")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
#[cfg(target_os = "windows")]
pub fn get_cputype() -> String {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("echo")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}