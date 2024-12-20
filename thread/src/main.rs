use std::thread;
use std::sync::{mpsc,Arc,Mutex};
use std::time::Duration;

fn main() {
    
    let handle_one = thread::spawn(|| {
        hello_thread(1)
    });

    let handle_two = thread::spawn(|| {
        hello_thread(2)
    });

    handle_one.join().unwrap();
    handle_two.join().unwrap();


    let result = thread::spawn(|| {
        Err::<&str,_>("Error 999")
    }).join().unwrap();

    match result {
        Ok(value) => println!("Thread finished successfully: {}", value),
        Err(err) => println!("Thread finished with an error: {}", err),
    }


    let result_one = thread::spawn(|| {
        10
    }).join().unwrap();

    let result_two = thread::spawn(move ||{
        result_one * 2
    }).join().unwrap();

    println!("result_one: {}", result_one);
    println!("result_two: {}", result_two);

    //channel
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hello from channel");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Received: {}", received);

    //Arc and Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result counter: {}", *counter.lock().unwrap());

    //Duration
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread: {}", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });

    handle.join().unwrap();
}

fn hello_thread(id: i32) {
    println!("Thread: {}", id);
}