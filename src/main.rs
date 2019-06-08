use std::ops::Drop;

#[derive(Debug)]
struct Parent(usize, Child, Child);

impl Drop for Parent {
    fn drop(&mut self) {
        println!("Droping {:?}", self);
    }
}

#[derive(Debug)]
struct Child(usize);

impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}


fn main() {
    println!("Hello, world!");
    // 7-3 所有権
    let p1 = Parent(1, Child(11), Child(12));
    {
        let p2 = Parent(2, Child(21), Child(22));
        println!("(a) p1: {:?}, p2: {:?}", p1, p2);
    }
    println!("(b) p1: {:?}", p1);
    let p3 = Parent(3, Child(31), Child(32));
    println!("(c) p1: {:?}, p3: {:?}", p1, p3);
    // 同じブロック内では後から導入された変数が先にスコープを抜けます
    /*
    (a) p1: Parent(1, Child(11), Child(12)), p2: Parent(2, Child(21), Child(22))
    Droping Parent(2, Child(21), Child(22))
    Dropping Child(21)
    Dropping Child(22)
    (b) p1: Parent(1, Child(11), Child(12))
    (c) p1: Parent(1, Child(11), Child(12)), p3: Parent(3, Child(31), Child(32))
    Droping Parent(3, Child(31), Child(32))
    Dropping Child(31)
    Dropping Child(32)
    Droping Parent(1, Child(11), Child(12))
    Dropping Child(11)
    Dropping Child(12)
    */
    // 7-5 ムーブセマンティクス
    let mut p1 = Parent(1, Child(11), Child(12));
    let p2 = p1;
    println!("p2: {:?}", p2);
    // println!("p1: {:?}", p1);
    // borrow of moved value: `p1` value borrowed here after move
    p1 = Parent(2, Child(21), Child(22));
    println!("p1: {:?}", p1);
    // 所有権のムーブを行うもの
    // - パターンマッチ
    // - 関数呼び出し
    // - コンストラクタ
    // - move クロージャ
    // 7-6 コピーセマンティクス
    sec7_6::m();
    // 7-7 借用 所有権を渡さずに値を貸し出す
    let mut p1 = Parent(1, Child(11), Child(12));
    f77(&p1); // 不変の参照
    f77_2(&mut p1); //可変の参照
    println!("p1: {:?}", p1);
    // 7-8 参照のライフタイムと借用規則
    // NLLがRust2018editionで入って落ち着いてる印象
    let mut map = HashMap::new();
    map.insert('h', "Hello".to_string());
    process_or_default('h', &mut map);
    // 7-9
    sec7_9::sec7_9();
    // 7-10
    sec7_10::sec7_10();
}
mod sec7_6;
mod sec7_9;
mod sec7_10;

fn f77(p: &Parent) {
    println!("{:?}", p);
}
fn f77_2(p: &mut Parent) {
    p.0 *= 2;
}

use std::collections::HashMap;
fn process_or_default(key: char, map: &mut HashMap<char, String>){
    // get_mutは可変の参照を得る
    match map.get_mut(&key) {
        Some(value) => value.push_str(", world"),
        None => {
            map.insert(key, Default::default());
        }
    }
}

