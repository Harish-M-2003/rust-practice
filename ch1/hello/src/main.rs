use std::io;

fn main() {

    let mut name = String::new(); // :: is an operator called which used to access static functions
    println!("Enter your name : ");
    io::stdin().read_line(&mut name).expect("Invalid input"); // read_line function is used to get a line of string as input

    if name == "harish" {
    
        println!("Hello, {}" , name);

    } else {
        println!("Your Name is  : {}" , name);
    }
}
