use sled::{Db, IVec};

fn main() {
    let tree = Db::open("my_db").unwrap();

    tree.insert(b"rick", vec![1, 2, 3]).unwrap();
    assert_eq!(tree.get(b"rick"), Ok(Some(IVec::from(vec![1, 2, 3]))));

    let expected = r#"{"One":1,"Two":2}"#;
    tree.insert(b"rick", expected).unwrap();
    /*
        assert_eq!(tree.get(b"rick"), Ok(Some(expected)));
    */

    assert_eq!(tree.get(b"rick"), Ok(Some(IVec::from(expected))));

    tree.flush().unwrap();
}
