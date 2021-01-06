use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::{Command, exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "nul" || args[1] == "/dev/null"
    {
        println!("{} does not exist in previous commit",args[2] );
        exit(0);
    }
    let path_s = Path::new(&args[1]);
    let temp_ext = path_s.extension().unwrap();
    #[cfg(target_os = "windows")]
    let temp_path = path_s.parent().unwrap();
    let temp_name = path_s.file_stem().unwrap();
    #[cfg(target_os = "windows")]
    let temp = format!(
        "{}{}_temp.{}",
        temp_path.display(),
        temp_name.to_str().unwrap(),
        temp_ext.to_str().unwrap()
    );
    #[cfg(not(target_os = "windows"))]
    let temp = format!(
        "./{}_temp.{}",
        temp_name.to_str().unwrap(),
        temp_ext.to_str().unwrap()
    );
    File::create(temp.as_str()).unwrap();
    fs::copy(&args[1], temp.as_str()).unwrap();
    #[cfg(target_os = "windows")]
    let clang_where = Command::new("where.exe")
        .arg("clang-format")
        .output()
        .expect("Failed");
    #[cfg(target_os = "windows")]
    let meld_where = Command::new("where.exe")
        .arg("meld")
        .output()
        .expect("Failed");
    #[cfg(not(target_os = "windows"))]
    let clang_where = Command::new("which")
        .arg("clang-format")
        .output()
        .expect("Failed");
    #[cfg(not(target_os = "windows"))]
    let meld_where = Command::new("which").arg("meld").output().expect("Failed");
    let mut clang = String::new();
    let mut meld = String::new();
    for i in clang_where.stdout {
        if i != 13 && i != 10 {
            clang.push(char::from(i));
        }
    }
    for i in meld_where.stdout {
        if i != 13 && i != 10 {
            meld.push(char::from(i));
        }
    }
    let clang_path = Path::new(clang.as_str());
    let meld_path = Path::new(meld.as_str());
    if temp_ext == "c" || temp_ext == "cpp" || temp_ext == "h" || temp_ext == "hpp" {
        Command::new(clang_path)
            .arg("-i")
            .arg("-style=file")
            .arg(&temp)
            .output()
            .expect("Failed to execute command");
    }
    Command::new(meld_path)
        .arg(&temp)
        .arg(&args[2])
        .output()
        .expect("Failed to execute command");
    #[cfg(not(target_os = "windows"))]
    fs::remove_file(temp).unwrap();
}
