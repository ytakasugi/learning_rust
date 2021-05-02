fn next_birthday(current_age: Option<u8>) -> Option<String> {
    // `current_age`が`None`の場合、`None`を返す。
    // `current_age`が`Some`の場合、内部の`u8`型の値が`next_age`に代入される。
    let next_age: u8 = current_age?;
    Some(format!("Next year I will be {}", next_age))
}

struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>
}

#[derive(Clone, Copy)]
#[allow(unused)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}

fn main() {
    println!("{}", next_birthday(Some(120)).unwrap());

    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
}
