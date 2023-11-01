use std::{thread, time::Duration, vec};

fn main() {
    // example 1
    let t = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    t.join().unwrap();
    println!();

    // example 2
    let v = vec![1, 2, 3];

    let t = thread::spawn(move || {
        println!("Vector = {:?}", v);
    });
    t.join().unwrap();
    println!();
}
