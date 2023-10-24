#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping smart pointer with data {}", self.data);
    }
}

fn main() {
    let p = CustomSmartPointer {
        data: String::from("hello world"),
    };
    println!("This is our smart pointer: {:?}", p);
    drop(p);
    println!("Dropped before end of program")
}
