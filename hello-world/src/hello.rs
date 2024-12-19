use std::io;


pub(crate) fn hello() {

    let mut name = String::new();

    println!("Please enter your name:");
    io::stdin().read_line(&mut name).expect("Failed to read line");
 
    match name.trim().is_empty() {
        true => {
            println!("I don't know you.");
        },
        false => {
            println!("Hello {}!", name);
            }
        }
}