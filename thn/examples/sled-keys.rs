// https://doc.rust-lang.org/rust-by-example/scope/lifetime/methods.html

use std::env;
use std::process;
use std::string::String;

use std::fs::File;
use std::io::BufReader;

use sled::Db;
use std::convert::TryInto;
use std::io::{BufRead, Error};

fn get_keys() -> Result<(), Error> {
    let tree = Db::open("sledb_hn").unwrap();

    // scan forward
    let mut keys = tree.iter().keys();
    // let key1 = iter.next().unwrap().unwrap();

    for key in keys {
        println!("{:?}", key);
    }

    // println!("{:?}",key);

    tree.flush().unwrap();
    Ok(())
}

fn main() {
    let _keys = get_keys();
}
