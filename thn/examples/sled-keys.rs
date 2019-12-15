use sled::Db;
use std::io::Error;
use std::str::from_utf8;

fn get_keys() -> Result<(), Error> {
    let tree = Db::open("sledb_hn").unwrap();
    let keys = tree.iter().keys();

    for key in keys {
        println!("{:?}", from_utf8(&key.unwrap()));
    }

    tree.flush().unwrap();
    Ok(())
}

fn main() {
    let _keys = get_keys();
}
