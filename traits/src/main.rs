use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait SummaryDefault {
    fn summarize_additional(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more... {})", self.summarize_additional())
    }
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

#[derive(Debug)]
struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: String,
    pub retweet: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct UsesDefaultTrait {}

impl SummaryDefault for UsesDefaultTrait {
    fn summarize_additional(&self) -> String {
        String::from("this is additional info")
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news!! {}", item.summarize())
}

// two traits required
pub fn notify2(item: &(impl Summary + Display)) {
    println!("Breaking news!! {}", item.summarize())
}

// trait bound
pub fn notify3<T: Summary + Display>(item: &T) {
    println!("Breaking news!! {}", item.summarize())
}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    5
}

// returning traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("test"),
        content: String::from("test"),
        reply: String::from("test"),
        retweet: String::from("test"),
    }
}

// conditional implement methods
#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Pair<T> {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x)
        } else {
            println!("The largest member is y = {}", self.y)
        }
    }
}

fn main() {
    let art = NewsArticle {
        headline: String::from("Pig escaped"),
        location: String::from("zoo"),
        author: String::from("Another pig"),
        content: String::from("some pig escaped from zoo, wtf?"),
    };
    println!("{}", art.summarize());

    let tweet = Tweet {
        username: String::from("afterlook"),
        content: String::from("traits work"),
        reply: String::from("xD"),
        retweet: String::from("nice"),
    };
    println!("{}", tweet.summarize());

    let default_struct_trait = UsesDefaultTrait {};
    println!("{}", default_struct_trait.summarize());

    notify(&art);
    notify(&tweet);
    println!("Number: {}", some_function(&6, &5));
    println!(
        "Summarizable sumarise: {}",
        returns_summarizable().summarize()
    );
}
