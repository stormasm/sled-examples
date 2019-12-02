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

    // Byte String Literals

    let k2 = "pete";
    let v2a = b"abc";
    let v2 = v2a.to_vec();
    tree.insert(k2, v2).unwrap();
    println!("{:?}", tree.get(k2));

    // as_bytes creates a Byte String Literal from a String
    // which you can then call to_vec on...

    let k3 = "stu";
    let v3a = "xyz";
    let v3b = v3a.as_bytes();
    let v3 = v3b.to_vec();
    tree.insert(k3, v3).unwrap();
    println!("{:?}", tree.get(k3));

    tree.flush().unwrap();
}
