#![allow(unused_imports, unused_variables)]

pub mod deserialize;
pub mod interface;
pub mod serialize;
pub mod types;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //if windows std::process::Command::new("cls").status().unwrap();

    //if mac.linux
    std::process::Command::new("clear").status().unwrap();
    match std::fs::metadata("test.yml") {
        Ok(_) => interface::home()?,
        Err(_) => interface::newcomer()?,
    }
    Ok(())
}