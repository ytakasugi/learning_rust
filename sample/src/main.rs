use std::fmt;

struct Password(String);

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.chars().map(|_| '*').collect::<String>())
    }
}

fn main() {
    let a = String::from("123456789");
    println!("{}", a);

    let password = Password(String::from("123456789"));
    println!("{}", password);
}