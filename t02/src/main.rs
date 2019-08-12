use sled::{Db, IVec};

fn main() {
//    let tree = Db::open("my_db").unwrap();
//    tree.set(b"yo!", b"v1".to_vec());
//    assert_eq!(tree.get(b"yo!"), Ok(Some(IVec::from(b"v1"))));
//    tree.flush();

    let tree = Db::open("my_db").unwrap();
    tree.insert(b"yo!", b"v1".to_vec());
    let val = tree.get(b"yo!");
//    println!("{}", val);

//    assert_eq!(tree.get(b"yo!"), Ok(Some(IVec::from(b"v1"))));
        tree.flush();

}
