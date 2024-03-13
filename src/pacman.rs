use crate::display::RoombaDisplay;
use std::process::Command;
use std::{io};

pub struct PackageManager {
    pub command_name: String,
    pub module_name: String
}

impl PackageManager {
    pub fn new(command_name: &str, module_name: &str) -> impl RoombaDisplay {
        return PackageManager {
            command_name: command_name.to_string(),
            module_name: module_name.to_string()
        };
    }
}

impl RoombaDisplay for PackageManager {
    fn name(&self) -> String {
        return self.module_name.clone();
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
                '`' => break,
                '0' => self.sys_upgrade(),
                '1' => self.clear_cache(),
                _ => println!("Invalid Option")
            }
        }
        return "".to_string();
    }
}

impl PackageManager {
    fn sys_upgrade(&self) {
        println!("Running 'sudo {} -Syu'", &self.command_name);
        
        let out = match Command::new("sudo")
            .arg(&self.command_name)
            .arg("-Syu")
            .output()
        {
            Ok(c) => c,
            Err(_) => {println!("Err running above command"); return;}
        };
        println!("{}", String::from_utf8_lossy(&out.stdout));
    }
    fn clear_cache(&self) {
        println!("Running 'sudo {} -Scc --noconfirm'", &self.command_name);
        let out = match Command::new("sudo")
            .arg(&self.command_name)
            .arg("-Scc")
            .arg("--noconfirm")
            .output()
        {
            Ok(c) => c,
            Err(_) => {println!("Err running above command"); return;}
        };
        println!("{}\nCache Cleared", String::from_utf8_lossy(&out.stdout));
    }
}
