mod cache;
mod failed;
mod pacman;

use crate::failed::{systemd, journal};

fn main() {
    top_interface();
}




fn top_interface() {
    println!("[0] Systemd  : {}", if systemd::status(){ "Ok" } else { "Err" } );
    println!("[1] Journal  : {}", if journal::status(){ "Ok" } else { "Err" } );
    println!("[2] Pacman   : {}", if systemd::status(){ "Ok" } else { "Err" } );
    println!("[3] Caches   : {}", if systemd::status(){ "Ok" } else { "Err" } );
}
