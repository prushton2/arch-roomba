pub mod systemd {
    use std::process::Command;
    pub fn status() -> bool {
        let out = match Command::new("systemctl")
                .arg("--failed")
                .output() 
        {
            Ok(c) => c,
            Err(_) => return false,
        };
        println!("{:?}", &out);
        let string = String::from_utf8_lossy(&out.stdout);
        return &string[36..37] == "0";
    }
}

pub mod journal {
    use std::process::Command;
    pub fn status() -> bool {
        let out = match Command::new("journalctl")
                .arg("-p 3")
                .arg("-xb")
                .output()
        {
            Ok(c) => c,
            Err(_) => return false
        };

        println!("{:?}", &out.);
        return true;
    }
}
