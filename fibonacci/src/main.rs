use std::{
    collections::HashMap,
    io::{self, Write},
};

fn main() {
    let mut to_calculate = String::new();
    print!("Give a fibonacci number to calculate: ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut to_calculate)
        .expect("couldn't read value");
    let to_calculate: i64 = to_calculate.trim().parse().expect("not a number!");
    let mut mem: HashMap<i64, i64> = HashMap::new();
    mem.insert(0, 0);
    mem.insert(1, 1);
    let result = fibonacci(to_calculate, &mut mem);
    println!("Result is: {result}");
}

fn fibonacci(num: i64, mem: &mut HashMap<i64, i64>) -> i64 {
    if mem.contains_key(&num) {
        return mem[&num];
    }
    let result = fibonacci(num - 1, mem) + fibonacci(num - 2, mem);
    mem.insert(num, result);
    result
}
