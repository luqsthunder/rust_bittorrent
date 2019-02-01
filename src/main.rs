use std::fs::File;
use std::io::prelude::Read;
use std::io::{Seek, SeekFrom};
use std::env;
use std::collections::{VecDeque, HashMap};

pub enum MetaInfoNodeType{
    Nothing,
    Integer,
    String,
    List,
    Dict,
}


pub struct MetaInfoNode {
    pub kind: MetaInfoNodeType,
    pub data: String,
    pub children: HashMap<String, MetaInfoNode>,
}

impl MetaInfoNode {
pub fn new() -> MetaInfoNode {
     MetaInfoNode{kind: MetaInfoNodeType::Nothing, data: String::new(),
                  children: HashMap::new()}
}

pub fn new_with(k : MetaInfoNodeType, d : String,
                c: HashMap<String, MetaInfoNode>) -> MetaInfoNode {

    MetaInfoNode{kind: k::Nothing, data: d, children: c}
}

pub fn decode_torrent_file(file : File) -> MetaInfoNode {
    // ideia é chamar recursivamente o decode chunk pra cada coisa.
}

//! Essa função lê um token inteiro, fazendo recurção se possivel para terminar
//! o nó, em geral ela lerá
fn decode_bencode(file: &mut File) -> (usize, MetaInfoNode) {
    let mut data = String::new();
    let mut children: HashMap<String, MetaInfoNode> = HashMap::new();

    // preparing to read single char
    let mut single_char:Vec<u8> = Vec::new();
    let read_char:&char = &(single_char[0] as char);
    single_char.resize(1, 0);
    let res = file.read_exact(&mut single_char.as_slice());
    // check for end of file;

    let mut read_size:usize = 1;


    // defining node type
    let tp_node: MetaInfoNodeType = match single_char as char {
        'i' => MetaInfoNodeType::Integer,
        '0'..'9' => MetaInfoNodeType::String,
        'l' => MetaInfoNodeType::List,
        'd' => MetaInfoNodeType::Dict,
    };

    if read_char == 'd' {
        let mut end_char:Vec<u8> = Vec::new();
        end_char.resize(1, '\0' as u8);

        while end_char[0] as char  != 'e' {
            // read key in file
            let size_n_key = decode_bencode(file);
            read_size += size_n_key.0;

            // set position of file at end of read key
            file.seek(SeekFrom::Start(read_size as u64));

            // read value
            let size_n_val = decode_bencode(file);
            read_size += size_n_val.0;

            // insert key, val into children hashmap and set position of file
            // at end of read value
            children.insert(size_n_key.1, size_n_val.1);
            file.seek(SeekFrom::Start(read_size as u64));

            //read if not 'e' back one char and decode the rest
            let res_d = file.read_exact(&mut single_char.as_slice());
            if single_char[0] as char != 'e' {
                file.seek(SeekFrom::Start(read_size as u64));
            }
            else {
                read_size += 1;
            }
        }
    }
    else if read_char == 'l' {
        // recurção
    }
    else if read_char >= '0' || read_char <= '9' {
        // for para ler ate o fim dos numeros, lê dois pontos, lê a quantidade

    }
    else if read_char == 'i' {

    }


    (read_size, MetaInfoNode::new_with(tp_node, data, children))
}

}





fn main() {
    let file_name = env::args().nth(0).unwrap();
    let mut torrent_file = File::open(file_name).unwrap();
}
