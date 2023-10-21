fn main() {
    let v1 = [1, 2, 3];
    let mut v1_iter = v1.iter();
    v1_iter.next();
    v1_iter.next();
    // part of iterator was used up
    for num in v1_iter {
        println!("num is: {}", num);
    }

    let total: i32 = v1.iter().sum();
    println!("Sum is {}", total);

    let total_every_plus_one: i32 = v1.iter().map(|e| e + 1).sum();
    println!("Sum every element + 1 is {}", total_every_plus_one);

    let new_v_plus_one: Vec<_> = v1.iter().map(|e| e + 1).collect();
    println!("New plus one: {:?}", new_v_plus_one);
}
