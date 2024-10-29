
// src/main.rs -> binary crate , a package can contains as many binary crates as possible
// src/lib.rs -> library crate , a package can contains only one library crate as possible

// A package is a collection crate
// A package contain Cargo.toml file
use crate::garden::vegetables::Tomato; // this kind of path type is known as absolute path
use crate::garden::get_garden_name; // this loaded the function which is decleared in crate garden
pub mod garden; // this tells the compiler to load the garden file

fn main() {
    let vegetable = Tomato {}; // Tomato is loaded from garden/vegetables/tomato
    vegetable.get_name();
    get_garden_name();
    println!("Hello, world!");
    println!("{:?}" , vegetable);
}
