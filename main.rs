#![windows_subsystem = "windows"]

use std::env;
use std::process::Command;
use std::thread;
use std::time;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut args_new: Vec<String> = Vec::new();

    let mut skip = true;

    for (pos, e) in args.iter().enumerate() {
        if skip {
            skip = false
        } else if e == "-time_offset" {
            args_new.push(e.to_string());

            if &args[pos + 1] == "-120" {
                args_new.push("-180".to_string());
                skip = true
            }

            if &args[pos + 1] == "300" {
                args_new.push("240".to_string());
                skip = true
            }
        } else {
            args_new.push(e.to_string())
        }
    }

    Command::new("archeage-original.exe")
        .args(args_new)
        .spawn()
        .expect("failed to execute process");

    thread::sleep(time::Duration::new(10, 0));
}
