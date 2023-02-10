use std::collections::hash_map;

mod utility{
    include!("utility/here/utility.rs");
}

fn main() {
    utility::hello();
    let kk:[i32;2] = [10,32];
    println!("Hello, world!");
}