mod cache;
mod failed;
mod pacman;

use crate::failed::{systemctl};

fn main() {
    top_interface();
}


fn top_interface() {
    println!("[0] Systemctl: {}", if systemctl::check_failed(){ "Ok" } else { "Err" } );
}
