mod cache;
mod failed;
mod pacman;
mod display;

use crate::failed::{Systemd, Journal};
use crate::cache::{Cache};
use crate::pacman::{PackageManager};
use crate::display::RoombaDisplay;

use std::io;

fn main() {

    let elements: [Box<dyn RoombaDisplay>; 5] = [Box::new(Systemd::new()), Box::new(Journal::new()), Box::new(Cache::new()), Box::new(PackageManager::new("pacman", "Pacman")), Box::new(PackageManager::new("paru", "Paru"))];

    loop {
        let selection: usize = top_interface(&elements);
        println!("{}", elements[selection].detail());
    }
}

fn top_interface(elements: &[Box<dyn RoombaDisplay>; 5]) -> usize {
    for i in 0..elements.len() {
        let element = &elements[i];
        println!("[{}] {}: {}", i, element.name(), element.status());
    }

    let stdin = io::stdin();
    let mut buffer;

    loop {
        buffer = String::from("");
        let _ = stdin.read_line(&mut buffer);
        
        match &buffer[0..buffer.len()-1].parse::<usize>() {
            Ok(c) => return *c,
            Err(c) => println!("{}", c),
        }
    }
}
