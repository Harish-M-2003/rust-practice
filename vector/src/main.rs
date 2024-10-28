fn main() {

    // let vector : Vec<i16> = Vec::new(); // this line creates a vector if type i16 , this is immutable vector
    // we cannot modify an immutable vector.
    // vectors can only store elements of same type
    let mut vector : Vec<i16> = Vec::new();
    
    for item in 0..=10 {
        vector.push(item + 20 );
    }
    println!("{:?}" , vector);
    println!("{}" , vector[1]); // this throws and error if index is out of bound.
    println!("{:?}",vector.get(1)); // the get method return a Some() struct , this handles the index out of bounde error

    // iteratin over the vector
    // for item in vector { // in this loop , the vector is moved , so from here vector is not valid
    //     println!("{}" , item);
    // }
    
    for item in &vector { // this is immutable reference , so we cannot modify the values inside the loop .
            println!("{}" , item);
    }

    println!("{:?}" , vector);

    // inorder to support vector to store elements of different type we need an enum

    #[derive(Debug)]
    enum Types {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let mut diff_vector : Vec<Types> = Vec::new();
    diff_vector.push(Types::Int(2));
    diff_vector.push(Types::Float(2.0));
    diff_vector.push(Types::Text("Harish".to_string()));
    
    println!("{:?}" , diff_vector);
    diff_vector.remove(1);
    println!("{:?}" , diff_vector);

    // like any other variable , a vector is droped when it goes out of scope
    // when a vector is dropped , all of it's contents will also be droped.

}
