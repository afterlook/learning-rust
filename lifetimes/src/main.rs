use std::fmt::Display;

fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }
    y
}

#[derive(Debug)]
struct StringHolder<'a> {
    some_str: &'a str,
}

impl<'a> StringHolder<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.some_str
    }
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        return x;
    }
    y
}

fn main() {
    // example of scope not living long enough
    // let r;
    //
    // {
    //     let x = 5;
    //     r = &x;
    // }
    //
    // println!("r: {}", r);

    // functions

    let s1 = String::from("abcd");
    let s2 = String::from("xyz");
    let result = longest_string(s1.as_str(), s2.as_str());
    println!("The longest string is: {}", result);

    // structs

    let s = String::from("Some string was here. It's not here anymore...");
    let first_sentence = s.split('.').next().expect("no dot present");
    let s_holder = StringHolder {
        some_str: first_sentence,
    };
    println!("This is our holder: {:?}", s_holder);
    println!("And this is value inside: {}", s_holder.some_str);

    let _s: &'static str = "Hello there";

    let l = longest_with_announcement(&s1, &s2, String::from("hello there"));
    println!("Longest from announcement: {}", l);
}
