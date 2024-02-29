pub mod systemctl {
    use std::process::Command;
    pub fn check_failed() -> bool {
        let out = match Command::new("systemctl")
                .arg("--failed")
                .output() 
        {
            Ok(c) => c,
            Err(_) => return false,
        };
        let string = String::from_utf8_lossy(&out.stdout);
 //       println!("{}", &string[36..37]);
        return &string[36..37] == "0";
    }
}

mod journalctl {

}
