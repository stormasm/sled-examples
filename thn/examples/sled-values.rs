// https://doc.rust-lang.org/rust-by-example/scope/lifetime/methods.html

use std::env;
use std::process;
use std::string::String;

use std::fs::File;
use std::io::BufReader;

use sled::Db;
use std::convert::TryInto;
use std::io::{BufRead, Error};

use std::str::from_utf8;

#[derive(Debug)]
struct SledToVec<'a> {
    key: &'a mut Vec<u32>,
    value: &'a mut Vec<String>,
}

impl<'a> SledToVec<'a> {
    fn is_even(num: u32) -> bool {
        (num) & 1 == 0
    }

    fn write_json_to_sled(key: String, data: String) -> Result<(), Error> {
        let tree = Db::open("sledb_hn").unwrap();
        let _x = tree.insert(key, data.as_str().as_bytes());
        tree.flush().unwrap();
        Ok(())
    }

    fn readdb(&mut self, dbname: String) -> Result<(), Error> {
        let tree = Db::open(dbname).unwrap();
        let keys = tree.iter().keys();

        for key in keys {
            let mykey = key.clone().unwrap();
            let xval = from_utf8(&mykey);
            println!("{:?}", xval);
        }

        tree.flush().unwrap();
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("You need to enter a db name");
        process::exit(1);
    }
    let dbname = &args[1];
    println!("In file {}", dbname);

    // Instantiate a SledToVec
    let mut ftv: SledToVec = SledToVec {
        key: &mut Vec::new(),
        value: &mut Vec::new(),
    };

    let _contents = SledToVec::readdb(&mut ftv, dbname.to_string());
}
