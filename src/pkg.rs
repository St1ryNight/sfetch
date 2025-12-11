use std::process::Command;
#[cfg(target_os = "macos")]
pub fn get_packages() -> String {
    // Check if brew exists
    let check_brew = Command::new("which")
        .arg("brew")
        .output()
        .expect("Failed to execute 'which' command");

    if !check_brew.status.success() {
        return "?".to_string();
    }

    let output = Command::new("sh")
        .arg("-c")
        .arg("brew list | wc -l")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
#[cfg(target_os = "freebsd")]
pub fn get_packages() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("pkg info | wc -l")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
#[cfg(target_os = "openbsd")]
pub fn get_packages() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("pkg_info | wc -l")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
#[cfg(target_os = "linux")]
pub fn get_packages() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo ?")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
#[cfg(target_os = "illumos")]
pub fn get_packages() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo ?")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
#[cfg(target_os = "windows")]
pub fn get_packages() -> String {
    let check_output = Command::new("powershell")
        .arg("-command")
        .arg("winget list | Select-Object -Skip 2 | Measure-Object -Line:w")
        .output()
        .expect("Failed to execute command");
    if !check_output.status.success() {
        return "?".to_string(); // Return "?" if winget is not available
    }
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("winget list | Select-Object -Skip 2 | Measure-Object -Line")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}