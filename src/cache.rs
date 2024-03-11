use crate::display::RoombaDisplay;
use std::process::Command;
use std::{env, io};

pub struct Cache {}

impl Cache {
    pub fn new() -> impl RoombaDisplay {
        return Cache {};
    }
}

impl RoombaDisplay for Cache {
    fn name(&self) -> String {
        return String::from("Cache");
    }

    fn status(&self) -> String {
        let out = match Command::new("du")
                .arg("-sh")
                .arg(format!("{}/.cache", env::home_dir().unwrap().display()))
                .output() 
        {
            Ok(c) => c,
            Err(_) => return "Err".to_string(),
        };
        let string = String::from_utf8_lossy(&out.stdout);
        let split = string.split("/").nth(0).unwrap();
        return split.to_string();
    }

    fn detail(&self) -> String {

        println!("Clear the cache?");

        let stdin = io::stdin();
        let mut buffer = String::from("");
        stdin.read_line(&mut buffer);

        if buffer.chars().nth(0).unwrap() == 'y' {
            let out = match Command::new("rm")
                    .arg("-rf")
                    .arg(format!("{}/.cache", env::home_dir().unwrap().display()))
                    .output() 
            {
                Ok(c) => c,
                Err(_) => return "Err".to_string(),
            };
        }
        return "Ok".to_string();
    }
}

