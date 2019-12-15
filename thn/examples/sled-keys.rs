use sled::Db;
use std::io::Error;

fn get_keys() -> Result<(), Error> {
    let tree = Db::open("sledb_hn").unwrap();
    let keys = tree.iter().keys();

    for key in keys {
        println!("{:?}", key);
    }

    tree.flush().unwrap();
    Ok(())
}

fn main() {
    let _keys = get_keys();
}
