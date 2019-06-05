#[derive(Copy, Clone, Debug)]
struct Parent(usize, Child);

#[derive(Copy, Clone, Debug)]
struct Child(usize);

pub fn m() {
    let p1 = Parent(76, Child(11));
    let p2 = p1;
    println!("p2: {:?}", p2);
    println!("p2: {:?}", p1);
    // 7-6-2 CopyトレイトとCloneトレイトの違い
    // Cloneトレイとはcloneメソッドを定義
    // Copy・・・暗黙的、ムーブの代わりにコピー、単純なことしかできない
    // Clone・・明示的、cloneメソッドを使ってコピー、自由に記述できる
}

