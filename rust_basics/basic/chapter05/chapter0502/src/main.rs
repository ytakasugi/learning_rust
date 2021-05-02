fn main() {
    // Vec<bool>型
    let v1 = vec![false, true, false];
    // ベクタv1の中身を表示
    for i in v1.iter() {
        println!("v1 = {}", i);
    }
    // Vec<f64>型
    let v2 = vec![0.0, -1.0, 1.0, 0.5];
    println!("v2.length = {}", v2.len());
    // ベクタv2の中身を表示
    for j in v2.iter () {
        println!("v2 = {}", j);
    }

    // 長さ100のベクタを作り、全要素を0i32で初期化する
    // （要素の型はCloneトレイトを実装していなければならない）
    let v3 = vec![0; 100];
    println!("v3.length = {}", v3.len());

    // ベクタは入れ子にできる。子の要素数はそれぞれが異なってもかまわない
    #[allow(unused_variables)]
    let v4 = vec![vec!['a', 'b', 'c'], vec!['d']];

    // ベクタは同じ型の要素の並び。異なる型の要素は持てない
    // let v5 = vec![false, 'a'];
    //   → error[E0308]: mismatched types

    // Vec<char>型
    let mut v6 = vec!['a', 'b', 'c'];

    for k in v6.iter() {
        println!("v6 = {}", k);
    }

    // Vec<char>型
    v6.push('d');
    v6.push('e');

    for k_af in v6.iter() {
        println!("v6_push_after = {}", k_af);
    }

    // 最後尾から値を取り出し
    assert_eq!(v6.pop(), Some('e'));

    // インデックス1の位置に要素を挿入
    v6.insert(1, 'f');

    for l in v6.iter() {
        println!("v6_insert_after = {}", l);
    }

    // インデックス2の要素を削除。返り値は削除した値
    v6.remove(2); 
    
    // v6の現在の値
    for m in v6.iter() {
        println!("v6_remove_after = {}", m);
    }

    // 別のベクタv7を作成
    let mut v7 = vec!['g', 'h'];
    // v6の最後尾にv7の全要素を追加
    v6.append(&mut v7);

    for n in v6.iter() {
        println!("v6_append = {}", n);
    }

    // v7は空になった（全要素がv6へ移動した）-> ループを回しても`println!で出力されない
    for p in v7.iter() {
        println!("v7 = {}", p);
    }

    // 固定長配列a8を作成
    let a8 = ['i', 'j'];
                 // v6の最後尾にa8の全要素を追加
    v6.extend_from_slice(&a8);

    for q in v6.iter() {
        println!("v6_extend_after = {}", q);
    }

    // a8は変更なし（a8の要素がコピーされた）
    for r in a8.iter() {
        println!("a8 = {}", r);
    }
}
