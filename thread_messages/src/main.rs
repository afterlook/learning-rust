use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi1"),
            String::from("hi2"),
            String::from("hi3"),
            String::from("hi4"),
            String::from("hi5"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi6"),
            String::from("hi7"),
            String::from("hi7"),
            String::from("hi9"),
            String::from("hi10"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got \"{}\" from thread using channels", received);
    }
}
