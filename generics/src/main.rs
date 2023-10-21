// FUNCS
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest_generic<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

// STRUCTS
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct PointTwo<T, U> {
    x: T,
    y: U,
}

#[derive(Debug)]
struct PointThree<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> PointThree<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointThree<X2, Y2>) -> PointThree<X1, Y2> {
        PointThree {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // FUNCS
    let number_list = vec![1, 2, 80, 4, 5];

    let mut largest_num = &number_list[0];

    for number in &number_list {
        if number > largest_num {
            largest_num = number;
        }
    }

    println!("The largest number is {}", largest_num);

    let largest_from = largest_i32(&number_list);
    println!("Largest from function {}", largest_from);

    let chars = vec!['a', 'b', 'z', 'g'];
    let largest_ch = largest_char(&chars);
    println!("Largest char is: {}", largest_ch);

    let largest_n_generic = largest_generic(&number_list);
    println!("Largest number generic is is: {}", largest_n_generic);

    let largest_ch_generic = largest_generic(&chars);
    println!("Largest character generic is is: {}", largest_ch_generic);

    // STRUCTS
    let _int_struct = Point { x: 1, y: 2 };
    println!("Int struct value of x: {}", _int_struct.x());
    let _float_struct: Point<f32> = Point { x: 1.0, y: 2.0 };
    println!(
        "Distance from origin: {}",
        _float_struct.distance_from_origin()
    );

    let _int_float_struct = PointTwo { x: 1, y: 2.0 };
    let _float_int_struct = PointTwo { x: 1.0, y: 2 };

    let p1 = PointThree { x: 1, y: 22.0 };
    let p2 = PointThree { x: 25.0, y: 5 };
    let p3 = p1.mixup(p2);
    println!("mixup of p1 and p2 is: {:?}", p3);
}
