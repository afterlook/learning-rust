#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, r: &Rectangle) -> bool {
        self.width > r.width && self.height > r.height
    }
}

fn main() {
    let r1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("R1: {:?}, area is: {}", r1, r1.area());
    dbg!(&r1);

    let r2 = Rectangle {
        width: 25,
        height: 40,
    };
    println!("Is r2 in r1? Answer: {}", r1.can_hold(&r2));
    let r3 = Rectangle {
        width: 50,
        height: 50,
    };
    println!("Is r3 in r1? Answer: {}", r1.can_hold(&r3));

    let r4 = Rectangle::square(50);
    dbg!(&r4);
}
