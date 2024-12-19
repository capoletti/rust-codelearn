extern crate rand;

struct Person {
    name: String,
    age: u8,
}

trait Voice{
    fn say(&self);
    fn has_voice(&self) -> bool;
}

impl Voice for Person {
    fn say(&self) {
        println!("{} says: Hello!", self.name);
    }

    fn has_voice(&self) -> bool {
        self.age > 0
    }
}

fn main() {
    
    let person = Person {
        name: String::from("John"),
        age: 20,
    };

    person.say();
    println!("Does {} have voice? {}", person.name, person.has_voice());

    match_example();
    //match_input();
    slice_string();
    string_methods();
    random();

}

fn match_example() {
    
    let number = 3;

    match number{
        1 => println!("One"),
        2 | 3 => println!("Two or Three"),
        4..=10 => println!("Four to Ten"),
        _ => println!("Something else")
    };

}

use std::io;
fn match_input(){

    let mut input = String::new();
    println!("Type something:");

    match io::stdin().read_line(&mut input){
        Ok(_) => print!("You typed: {}", input),
        Err(e) => println!("Error: {}", e)
    }

}

fn slice_string(){
    let value = String::from("Hello, World!");
    let hello = &value[0..5];
    let world = &value[7..12];
    println!("{} & {}", hello, world);
}

fn string_methods(){
   
    {
        let str = String::from("old");
        println!("{}", str);
        println!("{}", str.replace("old", "new"));
   }

   {
        let str = String::from("line1\nline2\nline3");
        for line in str.lines(){
            println!("({})", line);
        }
   }

   {
        let str = String::from("token+one+string");
        let tokens: Vec<&str> = str.split("+").collect();
        for token in tokens{
            println!("|{}|", token);
        }
   }
}

use rand::Rng;

fn random(){
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0., 1.);
    println!("Random number: {}", random_number);
}