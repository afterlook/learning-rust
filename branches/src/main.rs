fn main() {
    let number = 3;
    if number < 3 {
        println!("number is less than three");
    } else if number == 3 {
        println!("number is exactly three");
    } else {
        println!("this is else");
    }

    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("this is number: {number}");

    let mut number_of_times = 5;
    loop {
        println!("I'm in a loop");
        number_of_times = number_of_times - 1;
        if number_of_times == 0 {
            break;
        }
    }

    let mut counter = 0;
    let result = loop {
        counter = counter + 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("this is result: {result}");

    let mut first = 10;
    'first_loop: loop {
        let mut second = 0;

        loop {
            if first == 0 {
                break 'first_loop;
            }
            println!("inner loop");

            second = second + 1;
            if second == 5 {
                break;
            }
        }
        first = first - 1;
    }

    let mut cond = 5;
    while cond != 0 {
        println!("conditional while value: {cond}");
        cond = cond - 1;
    }

    let some_arr = [1, 2, 3, 5, 10];
    for num in some_arr {
        println!("printing num from some arr: {num}");
    }

    for idx in (0..5).rev() {
        let num_to_print = some_arr[idx];
        println!("looping from reverse: {num_to_print}")
    }
}
