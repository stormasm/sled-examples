use sled::{Db, IVec};

fn main() {
    let tree = Db::open("my_db").unwrap();
    tree.insert(b"rick", b"v1".to_vec()).unwrap();
    assert_eq!(tree.get(b"rick"), Ok(Some(IVec::from(b"v1"))));
    tree.flush().unwrap();
}
