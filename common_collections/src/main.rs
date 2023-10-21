use std::str::FromStr;

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    let first = &v[0];
    println!("First element {}", first);

    let _v_macro = vec![1, 2, 3];
    let v_better = [1, 2, 3, 4, 5];

    let third: &i32 = &v_better[3];
    println!("Third element: {}", third);

    let third_2: Option<&i32> = v.get(2);
    match third_2 {
        Some(num) => println!("There is num: {}", num),
        None => println!("No num"),
    }

    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i *= 2;
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("hello there general kenobi")),
        SpreadsheetCell::Float(23.5),
    ];
}
