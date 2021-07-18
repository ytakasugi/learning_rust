// 単語に連続して母音が3つ含まれるかどうかを判定したい
fn three_vowels(word: &String) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            // 母音が含まれていた時は、`vowel_count`をインクリメントする。
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                // `vowel_count`が3以上のときtrueを返す
                if vowel_count >= 3 {
                    return true
                }
            }
            // 上記以外のときは、インクリメントしない
            _ => vowel_count = 0
        }
    }
    // 単語に連続して母音が3つ含まれなかったときは、falseを返す
    false
}

fn main() {
    let ferris = "Ferris".to_string();
    let curious = "Curious".to_string();
    println!("{}: {}", ferris, three_vowels(&ferris));
    println!("{}: {}", curious, three_vowels(&curious));
}
