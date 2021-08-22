fn is_palindrome(x: i32) -> bool {
    // xが0より大きいかどうか判定
    if x < 0 || (x != 0 && x % 10 == 0) {
        return false;
    }

    let s = x.to_string();
    s[..s.len()]
        .chars()
        .zip(s[s.len() / 2..].chars().rev())
        .all(|(c1, c2)| c1 == c2)
}

fn main() {
    let x = 121;
    assert_eq!(is_palindrome(x), true);
}
