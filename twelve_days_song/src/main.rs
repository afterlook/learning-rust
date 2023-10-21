fn main() {
    let first_verse_diff = [
        "twelveth", "eleventh", "tenth", "ninghth", "eighth", "seventh", "sixth", "fifth",
        "fourth", "third", "second", "first",
    ];
    let rest = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves, and",
        "A partridge in a pear tree",
    ];

    let mut rest_idx = 10;
    for idx in (0..12).rev() {
        let verse_diff = first_verse_diff[idx];
        let first_verse =
            "On the ".to_string() + verse_diff + " day of Christmas, my true love sent to me";
        println!("{first_verse}");

        let mut second = 12;
        let mut rest_of_verse = String::new();
        while second != idx {
            second = second - 1;
            let next_verse = rest[second];
            rest_of_verse = next_verse.to_string() + "\n" + &rest_of_verse;
        }
        print!("{rest_of_verse}");
        rest_idx = rest_idx - 1;
        println!();
    }
}
