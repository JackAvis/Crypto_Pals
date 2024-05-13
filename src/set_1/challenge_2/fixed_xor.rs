use crate::set_1::challenge_1::hex_to_base64::hex_to_base64;
use hex;
use std::{cmp::min, ops::BitXor};

pub fn fixed_xor(input_bytes: &Vec<u8>, xor_input_bytes: &Vec<u8>) -> Vec<u8>{
    let mut xor_value: Vec<u8> = Vec::new();
    let min_length = min(input_bytes.len(), xor_input_bytes.len());
    for i in 0..min_length {
        let input_byte: u8 = *input_bytes.get(i).unwrap();
        let xor_input_byte: u8 = *xor_input_bytes.get(i).unwrap();
        let xor_byte = input_byte ^ xor_input_byte;
        xor_value.push(xor_byte);
    }
    return xor_value;
}