use std::io;

fn main() {
    println!("Input value in fahr:");
    let mut fahr = String::new();
    io::stdin()
        .read_line(&mut fahr)
        .expect("failed to read value");
    let fahr: f64 = fahr.trim().parse().expect("value not parsable to number");
    let celc: f64 = (fahr - 32.0) / 1.8;
    println!("Celcius value of your asked fahr value is: {:.2}", celc);
}
