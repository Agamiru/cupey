use Cupey::Cupey;

// use std::process::{exit, Command};

fn main() {

    let cupey = Cupey::new();
    cupey.copy_files().unwrap_or_else(|e| e.exit())

}