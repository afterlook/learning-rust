use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

#[derive(Debug)]
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![
            ShirtColor::Blue,
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Red,
        ],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("User with preference {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("User with preference {:?} gets {:?}", user_pref2, giveaway2);

    let _expensive_closure = |num: u32| -> u32 {
        println!("I'm a calculator");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    borrows_mutably();

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    // won't work, moved list to thread
    // println!("After defining closure: {:?}", list);

    let mut list = [
        Rectangle {
            width: 10,
            height: 3,
        },
        Rectangle {
            width: 3,
            height: 20,
        },
        Rectangle {
            width: 5,
            height: 5,
        },
    ];
    list.sort_by_key(|r| r.width);
    println!("Sorted by width {:#?}", list);
    list.sort_by_key(|r| r.height);
    println!("Sorted by height {:#?}", list);

    let mut a = String::from("hello");
    let b = &mut a;
    b.push_str(" world!");
    println!("{}", b);
    println!("{}", a);
}
