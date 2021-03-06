// https://doc.rust-lang.org/rust-by-example/scope/lifetime/methods.html

use std::env;
use std::io::Error;
use std::process;
use std::str::from_utf8;
use std::string::String;

#[derive(Debug)]
struct SledToVec<'a> {
    key: &'a mut Vec<String>,
    value: &'a mut Vec<String>,
}

impl<'a> SledToVec<'a> {
    #[allow(dead_code)]
    fn write_json_to_sled(key: String, data: String) -> Result<(), Error> {
        let tree = sled::open("sledb_hn").unwrap();
        let _x = tree.insert(key, data.as_str().as_bytes());
        tree.flush().unwrap();
        Ok(())
    }

    fn readdb(&mut self, dbname: String) -> Result<(), Error> {
        let tree = sled::open(dbname).unwrap();
        let keys = tree.iter().keys();

        for key in keys {
            let ckey = key.clone().unwrap();
            let xkey = from_utf8(&ckey).unwrap();
            self.key.push(xkey.to_string());
        }

        for i in 0..self.key.len() {
            let x = &self.key[i];
            println!("{}", x);

            // let y = tree.get(x).unwrap().unwrap();
            // println!("{:?}", from_utf8(&y));

            println!("{:?}", from_utf8(&tree.get(x).unwrap().unwrap()));
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
