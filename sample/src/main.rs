// 文字列から文字を検索する関数
// 文字列スライスとcharを引数にとり、`Option`型を返す
fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}

fn extension(file_name: &str) -> Option<&str> {
    find(file_name, '.').map(|i| &file_name[i+1..])
}

fn main() {
    let file_name = "foobar,rs";
    
    extension(file_name).unwrap_or("rs");
}