use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let file_name = env::args().nth(0).unwrap();
    let mut torrent_file = File::open(file_name).unwrap();

    let mut file_content:Vec<u8> = Vec::new();
    torrent_file.read_to_end(& mut file_content);
    let mut file_content_utf8 = String::from_utf8(file_content).unwrap_err();

    println!("{}", file_content_utf8.utf8_error().valid_up_to());
}
