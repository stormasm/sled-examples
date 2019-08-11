// use sled::{ConfigBuilder, Db, Result};

// fn basic() -> Result<()> {

use sled::{Db, IVec};

fn main() {

    let tree = Db::start_default("my_db").unwrap();
    tree.set(b"yo!", b"v1".to_vec());
    assert_eq!(tree.get(b"yo!"), Ok(Some(IVec::from(b"v1"))));

//    let config = ConfigBuilder::new().temporary(true).build();

    // let db = Db::start(config)?;
//    let tree = Db::start(config)?;

//    let config = ConfigBuilder::new().path(path).build();
//    let tree = track_any_err!(Db::start(config))?;

/*
    let tree = Db::open("./")?;

    let k = "joe";
    let v1 = 2;

    // set and get
    tree.insert(k, v1);
    assert_eq!(tree.get(&k), Ok(Some(v1)));
*/
/*

    // compare and swap
    tree.cas(k, Some(v1), Some(v2));

    // scan forward
    let mut iter = tree.scan(k);
    assert_eq!(iter.next(), Some(Ok((k, v2))));
    assert_eq!(iter.next(), None);

    // deletion
    tree.remove(&k);
*/
    // block until all operations are on-disk
    tree.flush();

}

/*
fn main() -> Result<()> {
    basic()?;
}
*/
