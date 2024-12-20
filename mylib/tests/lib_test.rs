use mylib::add;


#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn it_not_works() {
    let result = add(2, 2);
    assert_eq!(result, 5);
}
