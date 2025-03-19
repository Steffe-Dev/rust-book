unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

fn some_invalid_mem() {
    use std::slice;

    let address = 0x01234usize;
    let r = address as *mut i32;

    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    println!("{:?}", values);
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
        some_invalid_mem();
    }

    stuff()
}

fn stuff() {
    print!("forever ");

    let x = loop {
        print!("and ever ");
    };
}
