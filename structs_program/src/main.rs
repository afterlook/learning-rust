#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "This area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("Printing our rectangle: {:?}", rect1);
    dbg!(&rect1);

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect2);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
