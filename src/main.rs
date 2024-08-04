use std::io::{self};

fn main() {
    let mut six = String::new();
    println!("type name of you");
    let _out = io::stdin().read_line(&mut six);
    println!("Hello, {}!", six.trim_end());
}
