#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

#[derive(Debug)]
enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

#[derive(Debug)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
enum Message3 {
    Hello { id: i32 },
}

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color {}", color);
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple");
        } else {
            println!("Using orange");
        }
    } else {
        println!("Using blue as background color");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);

    while let Some(top) = stack.pop() {
        println!("This is num from stack = {}", top);
    }

    let v = ['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("`{}` is at index {}", value, index);
    }

    let x = 2;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("end x = {:?}, y = {}", x, y);

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        // this is inclusive pattern
        3..=5 => println!("three to five"),
        _ => println!("everything else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early char"),
        'k'..='z' => println!("late char"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    println!("Destructured vars of point are x = {}, y = {}", x, y);

    match p {
        Point { x, y: 0 } => println!("We're on the x axis with x = {}", x),
        Point { x: 0, y } => println!("We're on the y axis with y = {}", y),
        Point { x, y } => println!("Exhaustive case with x = {}, y = {}", x, y),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => println!("Quit variant"),
        Message::Move { x, y } => println!("Move variant with values x = {}, y = {}", x, y),
        Message::Write(s) => println!("Print variant with string = {}", s),
        Message::ChangeColor(x, y, z) => println!(
            "Change color variant with value x = {}, y = {}, z = {}",
            x, y, z
        ),
    }

    // nested matches
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("This is hsv with values h = {}, s = {}, v = {}", h, s, v)
        }

        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("This is rgb with values r = {}, g = {}, b = {}", r, g, b)
        }

        _ => println!("Anything else"),
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 10, y: 15 });

    let mut settings_value = Some(5);
    let new_setting_value = None;

    match (settings_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Cant perform overwrite"),
        _ => settings_value = new_setting_value,
    }

    if let Some(settings_value) = settings_value {
        println!("New settings value = {}", settings_value);
    } else {
        println!("New setting value does not exist");
    }

    let p = Point3D { x: 0, y: 0, z: 0 };

    match p {
        Point3D { x, .. } => println!("x = {}", x),
    }

    let numbers = (2, 3, 4, 5, 6, 7, 8);

    match numbers {
        (first, .., last) => println!("First num = {}, last num = {}", first, last),
    }

    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("Number is even, num = {}", x),
        Some(x) => println!("Number is not even, nub = {}", x),
        None => println!("There is no nunber"),
    }

    let msg = Message3::Hello { id: 56 };

    match msg {
        Message3::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message3::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message3::Hello { id } => println!("Found some other id = {}", id),
    }
}
