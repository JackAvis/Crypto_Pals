use std::fs;
use crate::set_1::{challenge_1::hex_to_base64::from_hex, challenge_3::single_byte_xor_cipher::{break_single_byte_xor_cipher, calc_english_score}};

pub fn detect_single_character_xor(strings: &String) -> Vec<u8> {
    let mut max_score = 0f32;
    let mut best_matching_decode  = Vec::new();
    for line in strings.split_whitespace() {
        let decoded_line = break_single_byte_xor_cipher(&from_hex(&line.as_bytes().to_vec()));
        let score: f32 = calc_english_score(&decoded_line);
        if (score > max_score){
            max_score = score;
            best_matching_decode = decoded_line;
        }
    }
    return best_matching_decode;
}


pub fn text_file_to_string(path: &String) -> String{
    let contents = fs::read_to_string(path).expect("File unable to be read");
    return contents;
}