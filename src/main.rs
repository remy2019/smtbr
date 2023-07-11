#![allow(unused_imports, unused_variables)]

pub mod deserialize;
pub mod interface;
pub mod serialize;
pub mod types;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exe = std::env::current_exe()?;
    let dir = exe.parent().expect("Executable must be in some directory");
    let yml = dir.join("test.yml");
    if cfg!(windows) {
        std::process::Command::new("cmd")
            .arg("/C")
            .arg("cls")
            .status()
            .unwrap();
    } else {
        std::process::Command::new("clear").status().unwrap();
    }

    match std::fs::metadata(yml) {
        Ok(_) => interface::home()?,
        Err(_) => interface::newcomer()?,
    }
    Ok(())
}
