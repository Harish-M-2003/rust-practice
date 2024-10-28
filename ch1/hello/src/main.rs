// use std::io;

// fn main() {

//     // let mut name = String::new(); // :: is an operator called which used to access static functions
//     // println!("Enter your name : ");
//     // io::stdin().read_line(&mut name).expect("Invalid input"); // read_line function is used to get a line of string as input

//     // if name == "harish" {
    
//     //     println!("Hello, {}" , name);

//     // } else {
//     //     println!("Your Name is  : {}" , name);
//     // }

//     // let mut n1 : String = String::from("");
//     // let mut n2 : String = String::from("");

//     // io::stdin().read_line(&mut n1).expect("Invalid input");
//     // io::stdin().read_line(&mut n2).expect("Invalid input");

//     // let n1 : isize = n1.parse().expect("REASON");
//     // println!("{n1}");

// }

// fn main() {
//     let _x: u32 = 5;
//     let mut _y: u32 = 5;

//     _y = _x;
//     let _v: u16 = 38_u8 as u16;
//     let _z : i32 = 10; // Type of z ? 

//     println!("Success!");
// }

use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),1); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2),3); 

    println!("Success!");
} 