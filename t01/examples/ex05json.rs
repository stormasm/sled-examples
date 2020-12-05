use sled::{IVec};

fn main() {
    let tree = sled::open("my_db").unwrap();

    tree.insert(b"rick", vec![1, 2, 3]).unwrap();
    assert_eq!(tree.get(b"rick"), Ok(Some(IVec::from(vec![1, 2, 3]))));

    let expected = r#"{"One":1,"Two":2}"#;
    tree.insert(b"storm", expected).unwrap();
    assert_eq!(tree.get(b"storm"), Ok(Some(IVec::from(expected))));

    tree.flush().unwrap();
}
