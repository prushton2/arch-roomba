use crate::display::RoombaDisplay;
use std::process::Command;

pub struct Systemd {}

impl Systemd {
    pub fn new() -> impl RoombaDisplay {
        return Systemd {};
    }
}

impl RoombaDisplay for Systemd {
    fn name(&self) -> String {
        return String::from("Systemd");
    }

    fn status(&self) -> bool {
        let out = match Command::new("systemctl")
                .arg("--failed")
                .output() 
        {
            Ok(c) => c,
            Err(_) => return false,
        };
        let string = String::from_utf8_lossy(&out.stdout);
        return &string[36..37] == "0";
    }

    fn detail(&self) -> String {
        let out = Command::new("systemctl")
                .arg("--failed")
                .output()
                .unwrap();
        return String::from_utf8_lossy(&out.stdout).to_string();
    }
}

pub struct Journal {}


impl Journal {
    pub fn new() -> Self {
        return Journal {};
    }
}

impl RoombaDisplay for Journal {
    fn name(&self) -> String {
        return String::from("Journal");
    }

    fn status(&self) -> bool {
        let out = match Command::new("journalctl")
                .arg("-p 3")
                .arg("-xb")
                .output()
        {
            Ok(c) => c,
            Err(_) => return false
        };

        return String::from_utf8_lossy(&out.stdout).split("\n").count() <= 1;
    }

    fn detail(&self) -> String {
        let out = Command::new("journalctl")
                .arg("-p 3")
                .arg("-xb")
                .output()
                .unwrap();
        return String::from_utf8_lossy(&out.stdout).to_string();
    }
}
