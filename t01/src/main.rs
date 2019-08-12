use sled::{Db, IVec};

fn main() {
    let tree = Db::start_default("my_db").unwrap();
    tree.set(b"yo!", b"v1".to_vec()).unwrap();
    assert_eq!(tree.get(b"yo!"), Ok(Some(IVec::from(b"v1"))));
    tree.flush().unwrap();
}
