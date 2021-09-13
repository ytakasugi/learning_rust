fn main() {
    let v = [0i32, 2, 3];

    let mut iter = v.iter().filter(|x| x.is_positive());

    assert_eq!(iter.next(), Some(&2));
}