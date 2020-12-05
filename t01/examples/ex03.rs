use sled::{IVec};

fn main() {
    let tree = sled::open("my_db").unwrap();
    tree.insert(b"rick", b"123 456 789".to_vec()).unwrap();
    assert_eq!(tree.get(b"rick"), Ok(Some(IVec::from(b"123 456 789"))));

    let result = tree.get(b"rick");
    println!("{:?}", result);

    let k1 = "bill";
    let v1 = b"abc def ghi".to_vec();
    tree.insert(k1, v1).unwrap();
    println!("{:?}", tree.get(k1));

    // Byte String Literals

    let k2 = "pete";
    let v2a = b"abc";
    let v2 = v2a.to_vec();
    tree.insert(k2, v2).unwrap();
    println!("{:?}", tree.get(k2));

    tree.flush().unwrap();
}
