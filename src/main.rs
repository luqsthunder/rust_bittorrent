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
    single_char.resize(1, 0);
    let res = file.read_exact(&mut single_char.as_slice());
    // check for end of file;

    let mut read_size = 1;


    // definindo o tipo do nó
    let tp_node: MetaInfoNodeType = match single_char as char {
        'i' => MetaInfoNodeType::Integer,
        '0'..'9' => MetaInfoNodeType::String,
        'l' => MetaInfoNodeType::List,
        'd' => MetaInfoNodeType::Dict,
    };

    if (single_char as char) == 'd' {
        let mut end_char = '\0';
        while end_char as char != 'e' {
            let size_n_key = decode_bencode(file);
            read_size += size_n_key.0;
            file.seek(SeekFrom(read_size));
        }
    }
    else if (single_char as char) == 'l' {
        // recurção
    }
    else if (single_char as char) >= '0' || (single_char as char) <= '9' {
        // for para ler ate o fim dos numeros, lê dois pontos, lê a quantidade
    }


    (read_size, MetaInfoNode::new())
}

}





fn main() {
    let file_name = env::args().nth(0).unwrap();
    let mut torrent_file = File::open(file_name).unwrap();
}
