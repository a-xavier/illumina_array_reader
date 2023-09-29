use std::collections::HashMap;
use std::fs::{File, read};
use std::io::Read;
use std::path::Path;
use flate2::read::GzDecoder;

/// Implement functions from 
/// https://github.com/HenrikBengtsson/illuminaio/blob/develop/R/readIDAT_nonenc.R
/// As a general rule:
/// We use the full binary as reference - &Vec<String> 
/// We pass a reader_head or offset to tell us where to start reading the binary and we use one of the
/// reading function to get the data: 
///     - Byte: 1 byte -> to u8    | vec version return vec of u32
///     - Short: 2 bytes -> to u16 | vec version return vec of u32
///     - Int: 4 bytes -> to u32   | vec version return vec of u32
///     - Long: 8 bytes -> to u64
///     - String:Custom function from illuminaio (directly copied)

pub fn get_offset_for_field(code_str_vector: &Vec<String>, offset_vector: &Vec<i64>, thing_to_search: &str) -> usize{
    let offset_index = get_index_of_element(&code_str_vector, thing_to_search.to_owned());
    return offset_vector.get(offset_index).unwrap().to_owned() as usize ;
}

pub fn read_byte(full_binary:  &Vec<u8>, reader_head: & mut usize ) -> u8 {
    let return_value = full_binary[*reader_head].to_owned();
    *reader_head += 1;
    return return_value;
}

// Return u8 to save space - Beads can't go up to 255
pub fn read_byte_vec(full_binary:  &Vec<u8>, reader_head: & mut usize, n: u32) -> Vec<u8> {
    let start_index =  (*reader_head..*reader_head + (n as usize )).step_by(1);
    let return_value: Vec<u8> = start_index.into_iter()
    .map(|x| {
        let result: u8 = full_binary[x];
        return result;
    })
    .collect();
    return return_value;
}

pub fn read_short(full_binary:  &Vec<u8>, reader_head: & mut usize ) -> u16 {
    let return_value = u16::from_le_bytes(full_binary[*reader_head..*reader_head+2].try_into().unwrap());
    *reader_head += 2;
    return return_value;
}

// Return u16 to save space - mean can't go up to 65000
pub fn read_short_vec(full_binary:  &Vec<u8>, reader_head: & mut usize, n: u32) -> Vec<u16> {
    let start_index =  (*reader_head..*reader_head + (n as usize * 2)).step_by(2);
    let return_value: Vec<u16> = start_index.into_iter()
    .map(|x| {
        let tmp: &[u8] =  &full_binary[x..x+2];
        let result: u16 = u16::from_le_bytes(tmp.try_into().unwrap());
        return result;
    })
    .collect();
    return return_value;
}

pub fn read_int(full_binary:  &Vec<u8>, reader_head: & mut usize) -> u32 {
    let return_value = u32::from_le_bytes(full_binary[*reader_head..*reader_head+4].try_into().unwrap());
    *reader_head += 4;
    return return_value;
}

pub fn read_int_vec(full_binary:  &Vec<u8>, reader_head: & mut usize, n: u32) -> Vec<u32> {
    let start_index =  (*reader_head..*reader_head + (n as usize* 4)).step_by(4);
    let return_value: Vec<u32> = start_index.into_iter()
    .map(|x| {
        let tmp: &[u8] =  &full_binary[x..x+4];
        let result: u32 = u32::from_le_bytes(tmp.try_into().unwrap());
        return result;
    })
    .collect();
    return return_value;
}

pub fn read_long(full_binary:  &Vec<u8>, reader_head: & mut usize) -> i64 {
    let return_value = i64::from_le_bytes(full_binary[*reader_head..*reader_head+8].try_into().unwrap());
    *reader_head += 8;
    return return_value;
}

pub fn read_bytes_to_read(full_binary: &Vec<u8>, reader_head: & mut usize) -> u64{
    let mut m = read_byte(full_binary, reader_head);
    let mut n = m % 128;
    let mut shift = 0;
    while m / 128 == 1{
        m = read_byte(full_binary, reader_head);
        shift =  shift + 7;
        let k = (m % 128) * 2^shift;
        n = n + k
    }
    return n.into();
}

pub fn read_string(full_binary:  &Vec<u8>, reader_head: & mut usize)-> String{
    let n = read_bytes_to_read(full_binary, reader_head) as usize;
    let slice = &full_binary[*reader_head..*reader_head + n];
    let caca = String::from_utf8(slice.try_into().unwrap()).unwrap();
    *reader_head += n;
    return  caca;
}

pub fn _read_unknown_bytes(full_binary: &Vec<u8>, reader_head: & mut usize) -> (u32, u8) {
    let n = read_bytes_to_read(full_binary, reader_head) as usize;
    let return_tuple = (n as u32, read_byte(full_binary, reader_head));//c(as.integer(n), readByte(con, n = n))
    *reader_head += 1;
    return return_tuple
}

pub fn read_magic(full_binary: &Vec<u8>, reader_head: & mut usize) -> String{
    let slice = &full_binary[0..4];
    let caca = String::from_utf8(slice.try_into().unwrap()).unwrap();
    *reader_head += 4;
    return caca;
}

pub fn read_version(full_binary:  &Vec<u8>, reader_head: & mut usize) -> i64{
    let return_value = read_long(full_binary, reader_head);
    return return_value
}

pub fn known_codes() -> HashMap<String, u16>{
    let mut hash:HashMap<String, u16> = HashMap::new();
    hash.insert(String::from("nSNPsRead"), 1000);
    hash.insert(String::from("IlluminaID"), 102);
    hash.insert(String::from("SD"), 103);
    hash.insert(String::from("Mean"), 104);
    hash.insert(String::from("NBeads"), 107);
    hash.insert(String::from("MidBlock"), 200);
    hash.insert(String::from("RunInfo"), 300);
    hash.insert(String::from("RedGreen"), 400);
    hash.insert(String::from("MostlyNull"), 401);
    hash.insert(String::from("Barcode"), 402);
    hash.insert(String::from("ChipType"), 403);
    hash.insert(String::from("MostlyA"), 404);
    hash.insert(String::from("Unknown.1"), 405);
    hash.insert(String::from("Unknown.2"), 406);
    hash.insert(String::from("Unknown.3"), 407);
    hash.insert(String::from("Unknown.4"), 408);
    hash.insert(String::from("Unknown.5"), 409);
    hash.insert(String::from("Unknown.6"), 410);
    hash.insert(String::from("Unknown.7"), 510);
    return hash;
}

pub fn known_codes_reverse() -> HashMap<u16, String>{
    let original_hash = known_codes();
    let mut new_hash : HashMap<u16, String> = HashMap::new();

    for (k, v) in original_hash{
        new_hash.insert(v, k);
    };
    return new_hash;
}

pub fn get_index_of_element(search_vector: &Vec<String>, s: String) -> usize{
    let index = search_vector.iter().position(|r| r == &s).unwrap();
    return index
}

pub fn get_file_name<T: ToString>(base: T, color: &str) -> String{
    // 1 - Check if the normal idat exists
    let basename = base.to_string();
    let plain_file = format!("{}_{}.idat", basename, color);
    let gzip_file = format!("{}_{}.idat.gz", basename, color);
    if Path::new(&plain_file).exists(){
        return plain_file
    } else if Path::new(&gzip_file).exists(){
        return gzip_file;
    }else {
        panic!("No file {} or {} found", &plain_file, &gzip_file)
    }
}

pub fn get_binary_from_file(path: &String) -> Vec<u8>{
    // Initialise output vector
    let mut output_vector : Vec<u8> = Vec::new();
    // IF GZIPPED
    if path.ends_with(".gz"){
        let file = File::open(path).unwrap(); //  We already know it exists
        let mut decoder = GzDecoder::new(file);
        decoder.read_to_end(&mut output_vector).unwrap(); // Dump data into output vector
    } else {
        output_vector = match read(path) {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
    }

    if output_vector.is_empty(){panic!("Could not read any data from Idat File")};

    return output_vector;
}