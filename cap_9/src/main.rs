use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    // read_from_file();
    can_fail()?;
    Ok(())
}

fn read_from_file() -> String {
    let f = File::open("hello.txt");
    let s = String::new();
    match f {
        Ok(_) => s,
        Err(_) => "Error".to_string()
    }
}

fn can_fail() -> Result<String, Box<dyn Error>> {
    Err(Box::from(String::from("Error")))
}