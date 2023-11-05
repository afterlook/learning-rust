use core::slice;

unsafe fn dangerous() {}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    let ptr = values.as_mut_ptr();
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
extern "C" fn call_from_c() {
    println!("Called rust function from C!");
}

static HELLO_WORLD: &str = "Hello, world global!";

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        // not possible as r1 is unmutable, compiler error
        // *r1 = 10;

        *r2 = 10;
        println!("This is r1 = {:?}", *r1);
        println!("This is r2 = {:?}", *r2);
    }

    unsafe {
        dangerous();
    }

    let mut v: [i32; 5] = [1, 2, 3, 4, 5];
    let (part_a, part_b) = split_at_mut(&mut v, 2);

    println!("Got parts. part a = {:?}, part b = {:?}", part_a, part_b);

    unsafe {
        println!("According to C language abs of -3 is {}", abs(-3));
    }

    call_from_c();

    println!("{}", HELLO_WORLD);

    add_to_count(5);
    unsafe {
        println!("Counter current value = {}", COUNTER);
    }
    add_to_count(10);
    unsafe {
        println!("Counter current value = {}", COUNTER);
    }
}
