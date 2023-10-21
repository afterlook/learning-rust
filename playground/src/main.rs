fn main() {
    let x = String::from("this is x");
    takes_ownership(x);
    println!("{}", x);
}

fn takes_ownership(x: String) {
    println!("{}", x);
}
