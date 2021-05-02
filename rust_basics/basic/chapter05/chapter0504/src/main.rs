use std::collections::HashMap;

fn main() {
    // HashMapの初期化
    let mut m1 = HashMap::new();    // または、with_capacity(要素数)

    // 要素を2つ追加する
    m1.insert("a", 1);  // キー: "a", バリュー: 1
    m1.insert("b", 3);
    assert_eq!(m1.len(), 2);
    // 確認
    println!("m1.len = {}", m1.len());

    // キーに対応する要素を取り出す
    assert_eq!(m1.get("b"), Some(&3));
    assert_eq!(m1.get("c"), None);
    // 確認
    println!("m1.get_b = {:?}", m1.get("b"));
    println!("m1.get_c = {:?}", m1.get("c"));

    // `d`が存在するなら、その参照を得る。存在しないなら`d`に対して0を登録してから参照を返す
    let d = m1.entry("d").or_insert(0);
    *d += 7;
    assert_eq!(m1.get("d"), Some(&7));
    // 確認
    println!("m1.get_d = {:?}", m1.get("d"));

    // HashMapを固定値で初期化したい場合、イテレーターのcollect()メソッドを使うのが簡単
    let m2 = vec![("a", 1), ("b", 3)].into_iter().collect::<HashMap<_, _>>();
    println!("m2.get = {:?}, {:?}", m2.get("a"), m2.get("b"));
}
