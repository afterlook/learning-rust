fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    print_something(&s);

    let s_alt = &mut s;
    s_alt.push_str(" This is cruel");
    print_something(s_alt);
    println!("{}", s);

    let mut s1 = String::from("hello");
    s1.push_str(", world!");
    let s2 = s1;
    println!("{}", s2);

    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    let s5 = String::from("hello");
    takes_ownership(s5);
    // this would not compile, s5 was borrowed to function and memory was freed
    // println!("{}", s5);
    let x = 5;
    makes_copy(x);
    // this is okay as x was on stack and copied over, 'move' didn't happen
    println!("from main: {}", x);

    let s6 = gives_ownership();
    println!("{}", s6);
    let s7 = String::from("s7");
    let s8 = takes_and_gives_back(s7);
    println!("{}", s8);

    let s9 = String::from("s9");
    let len = calculate_length(&s9);
    println!("this is len {} of {}", len, s9);

    let mut s10 = String::from("s10");
    change(&mut s10);
    println!("s10 after change: \"{}\"", s10);

    let mut s11 = String::from("s11");
    let _r1 = &mut s11;
    // cannot borrow twice
    // let r2 = &mut s11;
    // println!("{}{}", r1, r2);

    // let reference_to_nothing = dangle();
}

// Not allowed as s goes out of scope and we free memory, returned ref is corrupted
// fn dangle() -> &String {
//     let s = String::from("dangle");
//     &s
// }

fn change(s: &mut String) {
    s.push_str(", changed string");
}

fn calculate_length(s9: &String) -> usize {
    s9.len()
}

fn print_something(s: &String) {
    println!("{}", s);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(n: i32) {
    println!("from makes copy: {}", n);
}

fn gives_ownership() -> String {
    let s = String::from("gives ownershitp");
    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}
