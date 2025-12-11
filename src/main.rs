mod architecture;
mod cpu;
mod mem;
mod os;
mod pkg;
mod shell;
mod user;
mod kernel;
use owo_colors::OwoColorize;
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

    let username = user::get_username();
    let os_info = os::get_os();
    let memory = mem::get_memory();
    let (os, os_version) = kernel::get_os_info();
    let shell = shell::get_shell();
    let arch = architecture::get_cputype();
    let cpu = cpu::get_cpu();
    let pkg = pkg::get_packages();

    let side_text = [
        format!(
            "{}   -> {}",
            "USER".cyan(),
            username.trim(),
          //  hostname.trim()
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
        format!("{}   -> {}", "PKGS".blue(), pkg.trim()),
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



//fn get_hostname() -> String {
//    let output = Command::new("uname")
//        .arg("-n")
//        .output()
//        .expect("Failed to execute command");
//    String::from_utf8_lossy(&output.stdout).to_string()
//}

