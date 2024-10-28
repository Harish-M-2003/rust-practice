use std::collections::HashMap;
fn main() {

    let mut hash_map : HashMap<String , i8> = HashMap::new();
    hash_map.insert("A".to_string(), 1);
    hash_map.insert("b".to_string(), 1);
    hash_map.insert("c".to_string(), 1);
    hash_map.insert("d".to_string(), 1);
    hash_map.insert("e".to_string(), 1);
    hash_map.insert("f".to_string(), 1);
    hash_map.insert("g".to_string(), 1);
    hash_map.insert("h".to_string(), 1);

    println!("{:?}" , hash_map);
    println!("{:?}" , hash_map.get(&"A".to_string()));

    for (key, value) in &hash_map {
        println!("{key}: {value}");
    }


}
