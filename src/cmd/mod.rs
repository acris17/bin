use crate::sys;
use std::fs;

pub fn install(bin_dir: &str, file: &str) {
    if !sys::is_dir(bin_dir) {
        eprintln!("Error: The BIN_DIR environment variable is not set or valid");
        return;
    }
    if !sys::is_file(file) {
        eprintln!("Error: The program to install is not a valid file");
        return;
    }
    let Some(name) = sys::basename(file) else {
        eprintln!("Error: Could not get the file name from the program to install");
        return;
    };

    let dest = format!("{bin_dir}/{name}");
    if sys::exists(&dest) {
        let _ = fs::remove_file(&dest);
    }

    match fs::copy(file, &dest) {
        Ok(_) => println!("Installed: {dest}"),
        Err(e) => eprintln!("Error: Could not install: {e}"),
    }
}
pub fn uninstall(bin_dir: &str, name: &str) {
    if !sys::is_dir(bin_dir) {
        eprintln!("Error: The BIN_DIR environment variable is not set or valid");
        return;
    }

    let dest = format!("{bin_dir}/{name}");

    match fs::remove_file(&dest) {
        Ok(_) => println!("Uninstalled: {dest}"),
        Err(e) => eprintln!("Error: Could not uninstall: {e}"),
    }
}
pub fn list(bin_dir: &str) {
    let process = format!("ls {bin_dir}");

    sys::shell(&process);
}
