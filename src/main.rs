use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_s = Path::new(&args[1]);
    let temp_ext = path_s.extension().unwrap();
    let temp_path = path_s.parent().unwrap();
    let temp_name = path_s.file_stem().unwrap();
    let temp = format!(
        "{}{}_temp.{}",
        temp_path.display(),
        temp_name.to_str().unwrap(),
        temp_ext.to_str().unwrap()
    );
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
    File::create(temp.as_str()).unwrap();
    fs::copy(&args[1], temp.as_str()).unwrap();
    if temp_ext == "c" || temp_ext == "cpp" || temp_ext == "h" || temp_ext == "hpp" {
        Command::new(clang_path)
            .arg("-i")
            .arg(&temp)
            .output()
            .expect("Failed to execute command");
    }
    Command::new(meld_path)
        .arg(&temp)
        .arg(&args[2])
        .output()
        .expect("Failed to execute command");
}
