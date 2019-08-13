use sled::{Db, IVec};

fn main() {
    let tree = Db::open("my_db").unwrap();
    tree.insert(b"rick", b"123 456 789".to_vec()).unwrap();
    assert_eq!(tree.get(b"rick"), Ok(Some(IVec::from(b"123 456 789"))));

    let result = tree.get(b"rick");
    println!("{:?}", result);

    let k1 = "bill";
    let v1 = b"abc def ghi".to_vec();
    tree.insert(k1, v1).unwrap();
    println!("{:?}", tree.get(k1));

    tree.flush().unwrap();
}
