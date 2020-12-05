use sled::IVec;

fn main() {
    let tree = sled::open("my_db").unwrap();
    tree.insert(b"rick", vec![1, 2, 3]).unwrap();
    assert_eq!(tree.get(b"rick"), Ok(Some(IVec::from(vec![1, 2, 3]))));
    tree.flush().unwrap();
}
