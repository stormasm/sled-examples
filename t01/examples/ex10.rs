use std::str;

fn main() {
    let bytes: &[u8] = b"Hello, world!";
    println!("{:?}", bytes);
    let string = str::from_utf8(bytes).unwrap();
    println!("{}", string);

    let tree = sled::open("my_db").unwrap();
    let _ = tree.insert(b"rick", bytes);

    let result = tree.get(b"rick");
    println!("{:?}", result);

    //let string = str::from_utf8(result).unwrap();
    //println!("{}", string);
}
