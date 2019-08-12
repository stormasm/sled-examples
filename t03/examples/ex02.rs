//extern crate pagecache;
extern crate sled;

use sled::{ConfigBuilder, Db, Result};

fn basic() -> Result<()> {
    let config = ConfigBuilder::new().temporary(true).build();

    let db = Db::start(config)?;

    let k = b"k".to_vec();
    let v1 = b"v1".to_vec();
    //  let v2 = b"v2".to_vec();

    // set and get
    db.insert(k.clone(), v1.clone())?;
    assert_eq!(db.get(&k).unwrap().unwrap(), (v1.clone()));

    // deletion
    db.remove(&k)?;

    Ok(())
}

fn main() {
    basic().unwrap();
}
