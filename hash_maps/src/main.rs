use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Score of team blue is {}", score);

    for (key, value) in scores {
        println!("Team: {}, score: {}", key, value);
    }

    let field_name = String::from("field");
    let value = String::from("value");

    let mut hm = HashMap::new();
    hm.insert(field_name, value);
    // this won't compile, ownershir is in hashmap
    // println!("field: {}, value {}", field_name, value);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    for (k, v) in scores {
        println!("Conditional entry for team {}, and their score is {}", k, v)
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Counted words: {:?}", map);
}
