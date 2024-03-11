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

    fn status(&self) -> String {
        let out = match Command::new("systemctl")
                .arg("--failed")
                .output() 
        {
            Ok(c) => c,
            Err(_) => return "Err".to_string(),
        };
        let string = String::from_utf8_lossy(&out.stdout);
        return if &string[36..37] == "0" {"Ok".to_string()} else {"Err".to_string()};
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

    fn status(&self) -> String {
        let out = match Command::new("journalctl")
                .arg("-p 3")
                .arg("-xb")
                .output()
        {
            Ok(c) => c,
            Err(_) => return "Err".to_string()
        };

        return if String::from_utf8_lossy(&out.stdout).split("\n").count() <= 1 {"Ok".to_string()} else {"Err".to_string()};
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
