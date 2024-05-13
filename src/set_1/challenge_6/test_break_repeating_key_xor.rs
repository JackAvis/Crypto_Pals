use std::process::Command;
use crate::set_1::{challenge_1::hex_to_base64::from_hex, challenge_4::detect_single_character_xor::text_file_to_string, challenge_5::repeating_key_xor::repeating_key_xor, challenge_6::break_repeating_key_xor::{hamming_distance, break_repeating_key_xor}};

use super::break_repeating_key_xor;

#[cfg(test)]

#[test]
pub fn test_hamming_distance() {
    let input1 = "this is a test".as_bytes().to_vec();
    let input2 = "wokka wokka!!!".as_bytes().to_vec();
    let output = 37;
    assert_eq!(output, hamming_distance(&input1, &input2));
}


#[test]
pub fn test_break_reapeating_key_xor(){
    let file_path = String::from("./src/set_1/challenge_6/6.txt");
    let file_content= text_file_to_string(&file_path);
    let output = break_repeating_key_xor(&file_content.into_bytes());

    println!("--------Output-------");
    println!("{}", String::from_utf8(output).unwrap());
}



