use crate::set_1::challenge_1::hex_to_base64::hex_to_base64;
use hex;
use std::{cmp::min, ops::BitXor};

pub fn fixed_xor(input: &String, xor_input  : &String) -> String{
    let input_bytes: Vec<u8> = hex::decode(input).unwrap();
    let xor_input_bytes: Vec<u8> = hex::decode(xor_input).unwrap();
    let mut xor_value: Vec<u8> = Vec::new();
    let min_length = min(input_bytes.len(), xor_input_bytes.len());
    // xor = 1 when only 1 of the bits is 1
    for i in 0..min_length {
        let input_octet: u8 = *input_bytes.get(i).unwrap();
        let xor_input_octet: u8 = *xor_input_bytes.get(i).unwrap();
        let xor_octet = input_octet ^ xor_input_octet;
        xor_value.push(xor_octet);
    }
    return hex::encode(xor_value);
}

pub fn fixed_xor_bytes(input_bytes: &Vec<u8>, xor_input_bytes: &Vec<u8>) -> Vec<u8>{
    let mut xor_value: Vec<u8> = Vec::new();
    let min_length = min(input_bytes.len(), xor_input_bytes.len());
    // xor = 1 when only 1 of the bits is 1
    for i in 0..min_length {
        let input_octet: u8 = *input_bytes.get(i).unwrap();
        let xor_input_octet: u8 = *xor_input_bytes.get(i).unwrap();
        let xor_octet = input_octet ^ xor_input_octet;
        xor_value.push(xor_octet);
    }
    return xor_value;
}