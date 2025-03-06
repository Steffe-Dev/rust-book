use adder::add_two;

#[test]
fn it_adds_two_integration() {
    let res = add_two(2);
    assert_eq!(res, 4);
}
