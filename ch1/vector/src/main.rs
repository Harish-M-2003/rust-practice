use std::any::type_name;

fn type_of<T> (_ : T) -> &'static str {
    type_name::<T>()
}

fn main() {
    
    let n = vec![1,2,3,4];

    println!("{}",type_of(n.clone()));
    println!("{:?}" , &n[1..(n.len()-1)]);
    println!("{}" , n.len() == 4);

}
