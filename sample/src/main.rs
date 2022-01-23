fn total(init: i32, n: i32) -> i32 {
    (0..=n).fold(init, |sum, item| sum + item)
}

fn main() {
    let v = vec![1, 2, 3, 4];
    let mut iter = v.iter();

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), None);

    let x = &mut [1, 2, 4];
    for elem in x.iter_mut() {
        *elem += 2;
    }

    assert_eq!(x, &[3, 4, 6]);

    println!("{}", total(0, 10));
}