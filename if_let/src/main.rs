fn main() {
    let config_max = Some(3u8);
    // looking for one case like this bad
    match config_max {
        Some(max) => println!("maximum: {}", max),
        _ => (),
    }
    // if let better
    if let Some(max) = config_max {
        println!("maximum: {}", max)
    }
}
