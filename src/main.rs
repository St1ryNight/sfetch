mod architecture;
mod cpu;
mod mem;
mod os;
mod pkg;
use owo_colors::OwoColorize;
use std::process::Command;
fn main() {
    fetch();
}

fn fetch() {
    // here to change ascii art
    let asciiart = r#"    /\
   /  \
  /    \
  \    /
   \  /
    \/

"#;

    let username = get_username();
    let hostname = get_hostname();
    let os_info = os::get_os();
    let memory = mem::get_memory();
    let (os, os_version) = get_os_info();
    let shell = get_shell();
    let arch = architecture::get_cputype();
    let cpu = cpu::get_cpu();
    let pkg = pkg::get_packages();

    let side_text = [
        format!(
            "{}   -> {}@{}",
            "USER".cyan(),
            username.trim(),
            hostname.trim()
        ),
        format!(
            "{}     -> {} {}",
            "OS".magenta(),
            os_info.trim(),
            arch.trim()
        ),
        format!("{} -> {} MB", "MEMORY".green(), memory),
        format!(
            "{} -> {} {}",
            "KERNEL".yellow(),
            os.trim(),
            os_version.trim()
        ),
        format!("{}    -> {}", "CPU".white(), cpu.trim()),
        format!("{}   -> {}", "PKGS".bright_blue(), pkg.trim()),
        format!("{}  -> {}", "SHELL".purple(), shell.trim()),
    ];

    let ascii_lines: Vec<&str> = asciiart.lines().collect();
    let total_lines = ascii_lines.len();
    let text_lines = side_text.len();
    let start = if total_lines > text_lines {
        (total_lines - text_lines) / 2
    } else {
        0
    };

    for (i, line) in ascii_lines.iter().enumerate() {
        if i >= start && i < start + text_lines {
            let text = &side_text[i - start];
            println!("{:<8}  {}", line.blue(), text);
        } else {
            println!("{}", line.blue());
        }
    }
}

fn get_username() -> String {
    let output = Command::new("whoami")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn get_hostname() -> String {
    let output = Command::new("uname")
        .arg("-n")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn get_os_info() -> (String, String) {
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

fn get_shell() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo $SHELL")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
#[cfg(target_os = "linux")]
mod os {
    pub fn main() -> Result<(), Box<dyn std::error::Error>> {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Sorry.  This program isn't for you.  But, if you upgrade to Linux, you will be quite pleased.",
        )))
    }
}
