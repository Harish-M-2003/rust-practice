
// ownership rules
// 1. a value will have a owner
// 2. a value can have only one owner at a time
// 3. the variable is valid until it's scope.
// inorder to change anything in rust , we must use the mut keyword.

fn main() {
    // let age = 21; // simple types are stored on stack , so the memory is automatically managed by the compiler
    // let temp_age = age; // still age is valid
    // let mut array = [1,2,3]; // since the array size is know at compile time , it works similar to simple type in ownership
    // let mut temp_array = array; // this is a different copy , similar to deep copy
    // array[0] = 10;
    // temp_array[0] = 10;
    // let vector = vec![1,2,3]; // since vectors are created in heap , it follow ownership rules.
    // println!("{:?}" , array);
    // println!("{:?}" , temp_array); 
    // println!("age : {age} & temp : {temp_age}");
    // let name = String::from("Harish"); // String object created from string litral , name is valid from here
    // let temp = name; // name is moved in this line , so here after name is invalid
    // println!("{}",name); // this will throw an error at compile time , because name is invalid
    // println!("{temp}");
    // &temp -> immutable reference, similar to how variable are immutable.
    // let temp1 = &temp; // here just the reference is stored in the temp1 , so it is not a move
    // since this operation is not a move , temp1 and temp are still valid
    // println!("{temp1} & {temp}");

    // Reference
    // reference are used for passing value to functions or variables without moving it 
    // it does not has the ownership so the value pointed by a reference cannot be changed directly
    // for changing the value pointed by a reference we need a mutable reference (&mut reference)

    // Reference rules:
    // 1.) Multiple immutable reference can point to a value at a time (Read - Read no conflit).
    // 2.) only one mutable reference can point to a value at a time (because write conflict can occur in concurrent programming).
    // 3.) A reference will point to a valid value of a perticular type.

    let s1 = String::from("harish"); // -> this type of string declaration is know as string literal
    // string literals -> str
    println!("{s1}");
    let s2 = &s1;
    println!("s1 : {s1} and s2 : {s2}");


} // when the complier see this braket , it will call a function called 'drop' which will free the memory


