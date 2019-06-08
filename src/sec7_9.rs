pub struct ToyVec<T> {
    elements: Box<[T]>, // T型の要素を格納するBox
    len: usize,
}

impl<T: Default> ToyVec<T> {
    // newはキャパシティが0のToyVecをつくる
    pub fn new() -> Self {
        Self::with_capacity(0)
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0
        }
    }
    // T型がsize個できるBox<[T]>を返す
    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
/*
戻り値のusize型はCopyトレイトを実装していますので、所有権のムーブではなく、値がコピーされます。
=> 実装してなかったらムーブってこと。しんどいね。
*/
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    pub fn push(&mut self, element: T){
        if self.len == self.capacity() {
            self.grow();
        }
        self.elements[self.len] = element;
        self.len += 1;
    }
    pub fn get(&self, index: usize) -> Option<&T>{
        if index < self.len {
            Some(&self.elements[index])
        } else {
            None
        }
    }
    // ライフタイムを明示的に上書きする。
    pub fn get_or<'a>(&'a self, index: usize, default: &'a T) -> &'a T {
        self.get(index).unwrap_or(default)
    }
/*
・self.capacityが0のときは、allocate_in_heap(1)で長さ1のBox<[T]>を作成しself.elementsにセットする
・self.capacityが1以上のときは、allocate_in_heap(self.capacity()*2)で現在の2倍の長さのBox<[T]>を作成しself.elementsにセットする。既存の全要素を新しいBox<[T]>へムーブしたあと、古いBox<[T]>を破棄する
*/
    fn grow(&mut self) {
        if self.capacity() == 0 {
            self.elements = Self::allocate_in_heap(1);
        } else {
            let new_elements = Self::allocate_in_heap(self.capacity()* 2);
            let old_elements = std::mem::replace(&mut self.elements, new_elements);
            // 既存の全要素を新しい領域へムーブする
            // Vec<T>のinto_iter(self)なら要素の所有権が得られる
            for(i,elem) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = elem;
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            // let elem = self.elements[self.len];
            let elem = std::mem::replace(&mut self.elements[self.len], Default::default());
            Some(elem)
        }
    }
    
}
/*
やべぇ
static I0:i32 = 42; //static変数。'staticスコープを持つ
let mut s0:&'static str;
let s1 = "42"; //&str型。文字列リテラルへの参照（データは静的領域にある）
let s2 = 42.to_string();//String型（データはヒープ領域にある）
s0=s1; //文字列リテラルへの参照は'staticライフタイムを持つ
s0=&s2;//コンパイルエラー。String型から&'staticstrは作れない
//→error[E0597]:`s2`does not live long enough
*/

pub fn sec7_9(){
    let mut v = ToyVec::new();
    v.push("Java Finch".to_string());
    v.push("Budgeringer".to_string());
    let e = v.get(1);
    assert_eq!(e, Some(&"Budgeringer".to_string()));
    let mut v = ToyVec::new();
    v.push(100);
    let e = v.get(0);
    //v.push(200);
    assert_eq!(e,Some(&100));
}
