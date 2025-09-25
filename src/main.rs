use std::io;

fn main() {
    let mut x = String::new();
    println!("Enter string:");

    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");
    
    println!("String is {x}");

    println!("Ujjwal se ni hora commit");
}