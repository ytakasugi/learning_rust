use std::collections::HashMap;

fn roman_to_int(s: String) -> i32 {
    let t: HashMap<char, i32> = [
        ('M', 1000),
        ('D', 500),
        ('C', 100),
        ('L', 50),
        ('X', 10),
        ('V', 5),
        ('I', 1)
    ]
    .iter()
    .cloned()
    .collect();

    let mut res = 0;
    let mut pre = 'I';

    for c in s.chars().rev() {
        if t.get(&c).unwrap() < t.get(&pre).unwrap() {
            res -= t.get(&c).unwrap();
        } else {
            res += t.get(&c).unwrap();
        }
        pre = c;
    }
    res
}

fn main() {
    let s = "III".to_string();
    assert_eq!(roman_to_int(s), 3);
}
