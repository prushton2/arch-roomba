use crate::display::RoombaDisplay;
use std::process::Command;
use std::{io};

pub struct Pacman {}

impl Pacman {
    pub fn new() -> impl RoombaDisplay {
        return Pacman {};
    }
}

impl RoombaDisplay for Pacman {
    fn name(&self) -> String {
        return String::from("Pacman");
    }

    fn status(&self) -> String {
        return String::from("");
    }

    fn detail(&self) -> String {
        let stdin = io::stdin();
        let mut buffer;

        loop {
            buffer = String::from("");
            println!("[`] Exit\n[0] System Upgrade\n[1] Clear Cache\n");
            let _ = stdin.read_line(&mut buffer);

            match buffer.chars().nth(0).unwrap() {
                '`' => {break;}
                '0' => {
                    println!("Running 'sudo pacman -Syu'");
                    let out = match Command::new("sudo")
                        .arg("pacman")
                        .arg("-Syu")
                        .output()
                    {
                        Ok(c) => c,
                        Err(_) => {println!("Err"); continue;}
                    };
                    println!("{}", String::from_utf8_lossy(&out.stdout));
                }
                '1' => {
                    println!("Running 'sudo pacman -Scc --noconfirm'");
                    let out = match Command::new("sudo")
                        .arg("pacman")
                        .arg("-Scc")
                        .arg("--noconfirm")
                        .output()
                    {
                        Ok(c) => c,
                        Err(_) => {println!("Err"); continue;}
                    };
                    println!("{}\nCache Cleared", String::from_utf8_lossy(&out.stdout));
                }
                _ => {}
            }
        }
        return "".to_string();
    }
}
