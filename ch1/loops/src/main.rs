fn main() {


    /* For loop example 1*/
    println!("Print From 0 to 10 (exclusive)");
    for number in 0..10 {
        println!("{}" , number);
    }

    println!("Print from 0 to 10 (inclusive)");
    for number in 0..=10{
        println!("{}" , number);
    }

    // println!("Print from 10 to 0 (exclusive)");
    // for number in (0..10).reverse(){
    //     println!("{}" , number);
    // }

    println!("Print Letter in a word");
    for letter in "harish".chars() {
        println!("{}" , letter);
    }

    let mut i = 0;
    while i <= 5 {
        println!("While loop iteration : {}" , &i);
        i += 1;
    }

    i = 0;
    
    loop {
        println!("Hello, world!");
        if i == 5 {
            break;
        }
        i+=1; 
    }

}
