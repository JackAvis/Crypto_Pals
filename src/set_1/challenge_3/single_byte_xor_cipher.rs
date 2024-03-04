use std::collections::HashSet;
use crate::set_1::challenge_2::fixed_xor::{fixed_xor, fixed_xor_bytes};
use crate::set_1::challenge_1::hex_to_base64::hex_to_base64;


pub fn break_single_byte_xor_cipher(cipher: &String) -> String {
    let cipher_bytes = hex::decode(cipher).unwrap();
    let mut max_score: f32 = 0.0;
    let mut decoded_cipher = String::new();
    for possible_byte in 0u8..=255{
        let mut xor_bytes = Vec::new();
        for _ in 0..cipher.len(){
            xor_bytes.push(possible_byte);
        }
        let xor_result = fixed_xor_bytes(&cipher_bytes, &xor_bytes);
        let msg = String::from_utf8_lossy(&xor_result).to_string();
        let score = calc_english_score(&msg);
        if (score > max_score){
            max_score = score;
            decoded_cipher = msg;
        }
    }
    return decoded_cipher;
}

pub fn calc_english_score(message: &String) -> f32{
    let mut score: f32 = 0.0;
    for message_byte in message.bytes(){
        for (freq_byte, freq_score) in CHAR_FREQUENCIES.iter(){
            if (*freq_byte == message_byte){
                score += freq_score;
            }
        }
    }
    return score;

}

static CHAR_FREQUENCIES: [(u8, f32); 28] = [
    (b' ', 12.17), 
    (b'.', 6.57), 
    (b'a', 6.09),
    (b'b', 1.05),
    (b'c', 2.84),
    (b'd', 2.92),
    (b'e', 11.36),
    (b'f', 1.79),
    (b'g', 1.38),
    (b'h', 3.41),
    (b'i', 5.44),
    (b'j', 0.24),
    (b'k', 0.41),
    (b'l', 2.92),
    (b'm', 2.76),
    (b'n', 5.44),
    (b'o', 6.00),
    (b'p', 1.95),
    (b'q', 0.24),
    (b'r', 4.95),
    (b's', 5.68),
    (b't', 8.03),
    (b'u', 2.43),
    (b'v', 0.97),
    (b'w', 1.38),
    (b'x', 0.24),
    (b'y', 1.30),
    (b'z', 0.03),
];