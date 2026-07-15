use std::env::{set_current_dir, var};
use std::fs::create_dir_all;
use std::process::exit;

#[allow(unused)]
pub fn setup_dir() {
    let home = (if cfg!(windows) {
        var("USERPROFILE").unwrap()
    } else {
        var("HOME").unwrap()
    });
    if let Err(err) = create_dir_all(format!("{home}/dummy_shell_home")) {
        println!("{err}");
        exit(1);
    }
    let home = format!("{home}/dummy_shell_home");
    set_current_dir(home);
}
