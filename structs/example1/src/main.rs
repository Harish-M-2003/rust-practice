
#[derive(Debug)] // this is known as trait , it helps the println! macro to print this type .
struct Human {
    name : String,
    age : i8,
}

// #[derive(Debug)]
// struct Human_2 {
//     name : &str, // this require the concept lifetime.
//     age : i8,
// }

#[derive(Debug)]
struct Point (i8 , i8); // this type of struct is known as tuple struct

impl Human { // this block is used to include methods to a struct , which we usually do in a object oriented programming language

    // since rust is an expression based language , we can simply returnn values without using return keyword , for this we should avoid using semi colon at the end in the last statement in the function body.
    fn build(name : String , age : i32) -> Self {
        Self {
            name, // this is know as field shorthand , if the parament and the attribute name are same we can use this syntax
            age : age as i8 // since we are typecasting we need to specifiy both attribute and parameter even though they have the same name.
        }
    }

    fn build_from_struct(data : Human) -> Self { // this is an expression function , we should not place a semi colon at the end of a } in expression.
        Self {
            ..data // this is similar to js speard operator , is can placed only at the end in a block
        }
    }

    fn greet(&self) { // this is a method which can be invoked with the help of an instance
        println!("Hello!! {}." , self.name);
    }
}

fn consume(human : &Human){ // this kind of reference , is immutable , meaning we cannot change the value which this reference is pointing to.
    println!("{:?}" , human);
}

fn consume_mut(human : &mut Human){ // this kind of reference is mutable , we can modify the value of the reference we are pointin to.
    human.name = String::from("Doraemon");
    println!("{:?}" , human);
}

fn main() {

    // let harish = Human {
    //     name : String::from("Harish"),
    //     age : 21,
    // };

    // since we now created an associated function called build , which is similar to a static function 
    // we can use it like a constructor

    let harish = Human::build(String::from("Harish"), 21); // this build associated functoin can be used as a constructor
    let shinchan = Human::build_from_struct(Human {name : String::from("shinchan") , age : 5});
    let point = Point(10,10); // this is how we'll assign values to tuple struct

    // inorder to change the value of the struct , we need to use the mut keyword before the variebl

    let mut doraemon = Human::build(String::from("mon") , 10);

    // println!("{:?}" , doraemon);
    // doraemon.name = String::from("Doramon");
    // println!("{:?}" , doraemon);

    consume_mut(&mut doraemon); // since the consume_mut function takes mutable refernece as argunent , we should include the &mut 

    // print!("{:?}" , point); // print without adding a newline character
    println!("{:#?}" , point);
    println!("My Name {} and age is {}" , harish.name , harish.age);
    // consume(harish); // from this point , variable harish is not valid , because the ownership is moved to the parament of the consume function.
    consume(&harish); // inorder to prevent , changing of ownership we need to borrow the value , which can be done with '&' symbol.
    println!("Object  : {:?}" , harish); // :? -> used when we need to print the type which derives Debug traits.
    println!("Object  : {:#?}" , harish);// :#? -> prints the harish struct in a pretty format.
    harish.greet(); // greet method is implemented using the impl block
    shinchan.greet();

}
