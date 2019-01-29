use std::fs::File;
use std::io::prelude::*;
use std::str::Utf8Error;

use serde_json::{Result, Value};

fn main() {
    let mut torrent_file = match File::open("kingsman-golden-circle.torrent") {
        Ok(file) => file,
        Err(_err) => panic!("cannot open file"),
    };

    let mut file_content:Vec<u8> = Vec::new();
    torrent_file.read_to_end(& mut file_content);
    let mut file_content_utf8 = String::from_utf8(file_content).unwrap_err();

    println!("{}", file_content_utf8.utf8_error().valid_up_to());
    /*let v: Value = match serde_json::from_slice(file_content.as_slice()) {
      Ok(k) => k,
      Err(_e) => panic!(_e),
    };
    println!("{}", v["comment"]); */
}
