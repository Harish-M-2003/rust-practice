/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}

    Result is an enum with varients Ok(T) and Err(T)
 */

use std::io;
fn main() {
    
    // panic!("Hello world"); this is a macro which is used to throw an error

    // let mut  content : String = "".into();
    // print!("{:?}" , std::io::stdin().read_line(&mut content)); // this returns a Result enum
    // we can handle the errors using match statement

    // let _input_text = match io::stdin().read_line(&mut content) {
    //     Ok(t) => t,
    //     Err(e) =>  panic!("{}" , e),
    // };
    // io::stdin().read_line(&mut content).unwrap(); // this is a short for handling error Result enum
    // io::stdin().read_line(&mut content).expect("Invalid error"); // this is also a shortcut for handling error Result enum , but we can pass custom error message.
    let content = get_user_input().unwrap();
    println!("{}" , content);
    
}

fn get_user_input() -> Result<String , io::Error> {
    let mut  content : String = "".into();
    io::stdin().read_line(&mut content)?; // this question mark operator can only be used inside a functino which returns a Result or Option
    // ? is short for handling progating error
    Ok(content)
}

