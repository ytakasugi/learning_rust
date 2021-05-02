use std::marker::PhantomData;

// ジェネリックなタプル構造体。2つ目のパラメータは幽霊型
// 比較演算子(`==`)での比較を可能にする。
#[derive(PartialEq)]
#[derive(Debug)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

// 同様に構造体を定義
#[derive(PartialEq)]
#[derive(Debug)]
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

// 注意点:  ジェネリック型Aに対してはメモリが割り当てられているが、
//          Bには割り当てられていないため、計算に使うことはできない。

fn main() {
    // <char, f32>と型宣言されたPhantomTupleを作成
    let tuple: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    println!("{:?}", tuple);
    // <chr, f64>のPhantomTuple。 PhantomDataがいかなる浮動小数点でもないことに注目
    let tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);
    println!("{:?}", tuple2);

    // Type specified as `<char, f32>`.
    // <char, f32>の型が与えられた構造体を作成
    let struct1: PhantomStruct<char, f32> = PhantomStruct{
        first: 'Q',
        phantom: PhantomData,
    };

    println!("{:?}", struct1);

    // 同様に<char, f64>の構造体
    let struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    println!("{:?}", struct2);

}
