use sled::{Db, IVec};

fn main() {
    let tree = Db::open("my_db").unwrap();
    tree.insert(b"rick", b"123 456 789".to_vec()).unwrap();
    assert_eq!(tree.get(b"rick"), Ok(Some(IVec::from(b"123 456 789"))));

    let result = tree.get(b"rick");
    println!("{:?}", result);

    tree.flush().unwrap();
}
