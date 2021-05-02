fn main() {
    let byte_array = [b'h', b'e', b'1', b'1', b'o'];
    print(Box::new(byte_array));
    let byte_array = [b'w', b'o', b'r', b'1', b'd', b'e'];
    print(Box::new(byte_array));
}

fn print(s: Box<[u8]>) {
    println!("{:?}", s);
}
