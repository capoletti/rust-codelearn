fn main() {

    let numbers = vec![4, 1, 2, 6, 1, 4, 4, 5, 3, 0];
    println!("mean is {}", mean(&numbers));
    println!("median is {}", median(&numbers));
    println!("mode is {}", mode(&numbers));
    
    let numbers2 = vec![1,2,3,4,5];
    map_filter_reduce(&numbers2);
}

fn mean(numbers: &Vec<i32>) -> f64 {
    let sum: i32 = numbers.iter().sum();
    sum as f64 / numbers.len() as f64
}

fn median(numbers: &Vec<i32>) -> f64 {
    let mut numbers = numbers.clone();
    numbers.sort();
    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        (numbers[mid - 1] + numbers[mid]) as f64 / 2.0
    } else {
        numbers[mid] as f64
    }
}

use std::collections::HashMap;

fn mode(numbers: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for &number in numbers {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }
    let max = map.values().max().unwrap();
    *map.iter().find(|&(_, v)| v == max).unwrap().0
}


fn map_filter_reduce(numbers: &Vec<i32>){

    println!("{:?}", numbers.iter());

    let squares: Vec<_> = numbers.iter().map(|&x| x * x).collect();
    println!("Map - Squares: {:?}", squares);

    let evens: Vec<_> = numbers.iter().filter(|&x| x % 2 == 0).collect();
    println!("Filter - Evens: {:?}", evens); 

    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Fold - Sum: {}", sum);
}