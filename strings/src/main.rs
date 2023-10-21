fn main() {
    let data = "initial";

    let _s = data.to_string();
    let _s = "initial".to_string();
    let _s = String::from("initial");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("This is test: {}", s);

    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("This is another test {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("Laugh test: {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("String after addition {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("This is multiple concats with + op: {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("Tis is multiple concats using format: {}", s);

    for c in "Зд".chars() {
        println!("char of cyrylic {c}");
    }
    for c in "Зд".bytes() {
        println!("byte of cyrylic: {c}");
    }
}
