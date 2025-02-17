fn main() {
    shadow();
    mutable();
}

fn mutable() {
    let mut x = 5;
    println!("x is {x}");
    x = 6;
    println!("x is {x}");
}

fn shadow() {
    let x = 5;
    println!("x is {x}");
    let x = 6;
    println!("x is {x}");
}
