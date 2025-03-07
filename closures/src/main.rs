fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    print!("{list:?}");
    borrows_mutably();
    println!("After calling closure: {list:?}");
}
