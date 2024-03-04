use std::collections::HashSet;
use crate::set_1::challenge_2::fixed_xor::{fixed_xor, fixed_xor_bytes};
use crate::set_1::challenge_1::hex_to_base64::hex_to_base64;



pub fn single_byte_xor_cipher(encoded_string: &String) -> String {

    let encoded_bytes = hex::decode(encoded_string).unwrap();
    for i in 0u8..=255{
        let mut xor_bytes = Vec::new();
        for j in 0..encoded_string.len(){
            xor_bytes.push(i);
        }
        let xor_result = fixed_xor_bytes(&encoded_bytes, &xor_bytes);
        let msg = String::from_utf8_lossy(&xor_result);
        println!("{}", msg);
    }
    return String::from("output");
}



