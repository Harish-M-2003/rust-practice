fn main() {
    println!("Learn Datatypes!");

    // Numeric Datatype
    let _v1_ : &str = "str string"; // just storing the address of the string
    let _v2_ : u8 = 12; // throws compilation error , because unsigned number cannot have '-' sign.
    let _v3_ = 12u8; // this type of notaion is called type suffix.
    let _v4_ : i8 = 12; // this type of notaion is called type annotaion.
    
    let _v5_ : f32 = 12.0; // throws compilation error , when the mentioned type is floating point number f32 but binding value is integer
    let _v6_ : isize = 12; // takes the space according to the cpu architecture.

    // String Datatype
    let _v8_ : &str /*&str is optional*/ = "Harish"; // we cannot use type suffix syntax for strings
    
    // Array Datatype
    let mut _v10_ = [1,2,3,4,5,6]; // inorder to change the values or elements of an array , it should be declared as mutable.
    _v10_[0] = 10; // this binding will work only if the array is mutable array.
    
    let _v11_ = '1'; // single quotes are used to represent characters
    let _v12_ : char = '2'; //  stored the actual value

    println!("{}" , 56.7 / 32 as f64 );
    
}
