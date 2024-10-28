
use std::fs::File;
fn main() {
    let file = File::open("C:\\Users\\Harish\\Desktop\\test.py");
    
    match &file {
        Ok(f) => f,
        Err(_e) => panic!("File not found errror"),
    };

    println!("{:?}",file);
}
