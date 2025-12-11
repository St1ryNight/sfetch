mod architecture;
mod cpu;
mod mem;
mod os;
mod pkg;
mod shell;
mod user;
mod kernel;
use owo_colors::OwoColorize;

#[allow(dead_code)]
const LINUX_ASCII_COLOR: (u8, u8, u8) = (255, 215, 0);      
#[allow(dead_code)]
const MACOS_ASCII_COLOR: (u8, u8, u8) = (255, 255, 255);    
#[allow(dead_code)]
const WINDOWS_ASCII_COLOR: (u8, u8, u8) = (0, 120, 215);    
#[allow(dead_code)]
const OPENBSD_ASCII_COLOR: (u8, u8, u8) = (255, 200, 46);   
#[allow(dead_code)]
const FREEBSD_ASCII_COLOR: (u8, u8, u8) = (204, 0, 0);      
#[allow(dead_code)]  
const ILLUMOS_ASCII_COLOR: (u8, u8, u8) = (255, 102, 0);    

fn main() {
    fetch();
}

fn fetch() {
    // here to change ascii art
    let ascii_art = get_ascii_art();
    let ascii_color = get_ascii_color();

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

    let ascii_lines: Vec<&str> = ascii_art.lines().collect();
    let total_lines = ascii_lines.len();
    let text_lines = side_text.len();
    let start = if total_lines > text_lines {
        (total_lines - text_lines) / 2
    } else {
        0
    };

    let max_width = ascii_lines.iter().map(|line| line.len()).max().unwrap_or(0);

    for (i, line) in ascii_lines.iter().enumerate() {
        if i >= start && i < start + text_lines {
            let text = &side_text[i - start];
            println!("{:<width$}  {}", line.truecolor(ascii_color.0, ascii_color.1, ascii_color.2), text, width = max_width);
        } else {
            println!("{}", line.truecolor(ascii_color.0, ascii_color.1, ascii_color.2));
        }
    }
}

#[cfg(target_os = "linux")]
fn get_ascii_color() -> (u8, u8, u8) {
    LINUX_ASCII_COLOR
}

#[cfg(target_os = "macos")]
fn get_ascii_color() -> (u8, u8, u8) {
    MACOS_ASCII_COLOR
}

#[cfg(target_os = "windows")]
fn get_ascii_color() -> (u8, u8, u8) {
    WINDOWS_ASCII_COLOR
}

#[cfg(target_os = "openbsd")]
fn get_ascii_color() -> (u8, u8, u8) {
    OPENBSD_ASCII_COLOR
}

#[cfg(target_os = "freebsd")]
fn get_ascii_color() -> (u8, u8, u8) {
    FREEBSD_ASCII_COLOR
}

#[cfg(target_os = "illumos")]
fn get_ascii_color() -> (u8, u8, u8) {
    ILLUMOS_ASCII_COLOR
}

#[cfg(target_os = "linux")]
fn get_ascii_art() -> &'static str {
    r#"    ___   
   [..,|  
   [<> |  
  / __` \ 
 ( /  \ {| 
 /\ __)/,)
(}\____\/ "#
}

#[cfg(target_os = "windows")]
fn get_ascii_art() -> &'static str {
r#"$1lllllll  $2lllllll
$1lllllll  $2lllllll
$1lllllll  $2lllllll

$3lllllll  $4lllllll
$3lllllll  $4lllllll
$3lllllll  $4lllllll
    "#
}

#[cfg(target_os = "macos")]
fn get_ascii_art() -> &'static str {
    r#"          .:'
      __ :'__
   .'`__`-'__``.
  :__________.-'
  :_________:
  : _________`-;
    `.__.-.__.
      "#
}

#[cfg(target_os = "openbsd")]
fn get_ascii_art() -> &'static str {
    r#"       ____
    \-     -/
 \_/         \
 |        O O |
 |_  <   )  3 )
 / \         /
    /-_____-\
    "#
}

#[cfg(target_os = "freebsd")]
fn get_ascii_art() -> &'static str {
    r#"/\,-'''''-,/\
\_)       (_/
|           |
|           |
 ;         ;
  '-_____-'
   "#
}

#[cfg(target_os = "illumos")]
fn get_ascii_art() -> &'static str {
    r#"       .   .;   .
   .   :;  ::  ;:   .
   .;. ..      .. .;.
..  ..             ..  ..
 .;,                 ,;.

    "#
}

//fn get_hostname() -> String {
//    let output = Command::new("uname")
//        .arg("-n")
//        .output()
//        .expect("Failed to execute command");
//    String::from_utf8_lossy(&output.stdout).to_string()
//}

