struct MyOption<T>(Option<T>);

impl<T> MyOption<T> {
    fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self.0 {
            Some(value) => value,
            None => f(),
        }
    }
}

fn main() {

    //closure
    let add_one_v1 = |x: u32| -> u32 {x + 1};
    let add_one_v2 = |x: u32| {x + 1};
    let add_one_v3 = |x| x + 1;

    println!("calculation 1: {}", add_one_v1(1));
    println!("calculation 2: {}", add_one_v2(2));
    println!("calculation 3: {}", add_one_v3(3));

    let fallback = || 50;

    let some_value = MyOption(Some(10));
    let result_some = some_value.unwrap_or_else(fallback);
    println!("result_some: {}", result_some);

    let none_value = MyOption(None);
    let result_none = none_value.unwrap_or_else(fallback);
    println!("result_none: {}", result_none);

    iterator();


    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 12, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_size(shoes, 10);
    println!("shoes in my size: {:?}", in_my_size);

}

#[derive(PartialEq, Debug)]
struct Shoe{
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn iterator() {
    let vector = vec![1, 2, 3];
    let mut v_iter = vector.iter();  

    println!("Iter 1 {}", v_iter.next() == Some(&1));
    println!("Iter 2 {}", v_iter.next() == Some(&2));
    println!("Iter 3 {}", v_iter.next() == Some(&3));
    println!("Iter 0 {}", v_iter.next() == None);
}