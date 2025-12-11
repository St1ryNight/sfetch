use std::process::Command;
#[cfg(target_os = "macos")]

pub fn get_os_info() -> (String, String) {
    let os_output = Command::new("uname")
        .output()
        .expect("Failed to execute command");
    let os = String::from_utf8_lossy(&os_output.stdout).to_string();

    let version_output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Failed to execute command");
    let version = String::from_utf8_lossy(&version_output.stdout).to_string();

    (os, version)
}
#[cfg(target_os = "linux")]

pub fn get_os_info() -> (String, String) {
    let os_output = Command::new("uname")
        .output()
        .expect("Failed to execute command");
    let os = String::from_utf8_lossy(&os_output.stdout).to_string();

    let version_output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Failed to execute command");
    let version = String::from_utf8_lossy(&version_output.stdout).to_string();

    (os, version)
}
#[cfg(target_os = "openbsd")]

pub fn get_os_info() -> (String, String) {
    let os_output = Command::new("uname")
        .output()
        .expect("Failed to execute command");
    let os = String::from_utf8_lossy(&os_output.stdout).to_string();

    let version_output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Failed to execute command");
    let version = String::from_utf8_lossy(&version_output.stdout).to_string();

    (os, version)
}
#[cfg(target_os = "illumos")]

pub fn get_os_info() -> (String, String) {
    let os_output = Command::new("uname")
        .output()
        .expect("Failed to execute command");
    let os = String::from_utf8_lossy(&os_output.stdout).to_string();

    let version_output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Failed to execute command");
    let version = String::from_utf8_lossy(&version_output.stdout).to_string();

    (os, version)
}
#[cfg(target_os = "freebsd")]

pub fn get_os_info() -> (String, String) {
    let os_output = Command::new("uname")
        .output()
        .expect("Failed to execute command");
    let os = String::from_utf8_lossy(&os_output.stdout).to_string();

    let version_output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Failed to execute command");
    let version = String::from_utf8_lossy(&version_output.stdout).to_string();

    (os, version)
}
#[cfg(target_os = "windows")]
pub fn get_os_info() -> (String, String) {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("(Get-CimInstance Win32_OperatingSystem).Version")
        .output()
        .expect("Failed to execute command");
    
    let result = String::from_utf8_lossy(&output.stdout);
    let parts: Vec<&str> = result.split('|').collect();
    
    let os = parts.get(0).unwrap_or(&"").to_string();
    let version = parts.get(1).unwrap_or(&"").to_string();

    (os, version)
}
