use traits_continued::{Button, Draw, Screen};

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing select box {:?}", self);
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 50,
                height: 50,
                options: vec![String::from("opt1")],
            }),
            Box::new(Button {
                width: 100,
                height: 100,
                label: String::from("label1"),
            }),
        ],
    };
    screen.run()
}
