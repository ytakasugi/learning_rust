use proconio::input;
use proconio::marker::Chars;

fn main() {
	input! {
		s: [String; 3],
		t: Chars,
	}

	for u in t {
		let n = u as usize - 48;
		print!("{}", s[n-1]);
	}
}