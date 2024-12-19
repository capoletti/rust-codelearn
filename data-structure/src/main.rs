fn main() {
      
    tupla_example();
    enum_example(); 
    block_code();
    shadowing();
    static_unsafe();
    references();
    structs();
    strings();
    arrays();
    vector();
    hashmap();
}

fn tupla_example(){

    let tupla = (1, 2, 3, ("a", "b", "c"));
    let (a, b, c, d) = tupla;

    println!("tuplas");
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {:?}", d);

}

#[derive(Debug)]
enum Gender{
    Male, Female
}

enum Color{
    Red(u32), Blue(u32)
}

fn enum_example(){

    let male = Gender::Male;
    let female = Gender::Female;

    println!("enum");
    println!("{:?}", male);
    println!("{:?}", female);

    match male{
        Gender::Male => println!{"is male"},
        Gender::Female => println!{"is female"}
    }

    let item_r = Color::Red(1);
    let item_b = Color::Blue(10);

    match item_r{
        Color::Red(value) => println!("Red: {}", value),
        Color::Blue(value) => println!("Blue: {}", value)
    }

    match item_b{
        Color::Red(value) => println!("Red: {}", value),
        Color::Blue(value) => println!("Blue: {}", value)
    }

}

fn block_code(){

    println!("block_code");

    let x = 10;
    {
        let y = 20;
        println!("x: {}, y: {}", x, y);
    }
}

fn shadowing(){

    println!("shadowing");

    let x = 10;
    {
        println!("x: {}", x);
        let x = 15;
        println!("x: {}", x);
    }
    println!("x: {}", x);
}

fn static_unsafe(){

    println!("static_unsafe");

    static mut X: i32 = 10;

    unsafe{
        X = 20;
    }
}

fn references(){

    println!("references");

    let mut x = 10;
    let y = &mut x; //borrow x to 
    *y += 1;

    println!("y: {}", y);
}

struct Point{
    name: String,
    x: i32,
    y: i32
}

struct Points(String, i32, i32);

fn structs(){

    println!("structs");

    let p1 = Point{name: "Point One".to_string(), x: 10, y: 20};
    print_structs(&p1);//pass by reference

    let p2 = Points("Point Two".to_string(), 10, 20);
    println!("name: {}, p.x: {}, p.y: {}", p2.0, p2.1, p2.2);

}

fn print_structs(p: &Point){//pass by reference
    println!("name: {}, p.x: {}, p.y: {}", p.name, p.x, p.y);
}

fn strings(){

    println!("strings");

    let s1 = "Hello!"; //slice
    let s2 = s1.to_string();
    let s3 = String::from("Hello!");

    println!("{}, {}", s2, s3);
}

fn arrays(){

    println!("arrays");

    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5];

    println!("{}", a[2]);

    for i in 0..a.len(){
        print!("{}, ", a[i]);
    }
    println!();

    for n in b.iter(){
        print!("{}, ", n);
    }
    println!();

    println!("{:?}", b);
    println!("{:?}", c);
}

fn vector(){
    
        println!("vector");
    
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        v.remove(1);
    
        println!("{:?}", v);
    
        let v2 = vec![6, 2, 8, 9,10];
        println!("{:?}", v2);
    
        let v3 = vec![0; 5];
        println!("{:?}", v3);
    
        let first = &v2[0];
        println!("{}", first);
    
        match v2.get(1){
            Some(second) => println!("{}", second),
            None => println!("None")
        }
    
        for i in &v2{
            print!("{}, ", i);
        }
        println!();
    
        for i in &mut v{
            *i += 50;
        }
    
        for i in &v{
            print!("{}, ", i);
        }
        println!();
}

fn hashmap(){

    println!("hashmap");

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert("Math", 90);
    scores.insert("Science", 80);
    scores.insert("History", 70);

    match scores.get("Math"){
        Some(score) => println!("Math: {}", score),
        None => println!("None")
    }

    match scores.get("English"){
        Some(score) => println!("Math: {}", score),
        None => println!("None")
    }

    scores.remove("History");
    println!("{} & {}", scores.contains_key("History"), scores.contains_key("Science"));

}