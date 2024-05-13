use crate::set_1::challenge_3::single_byte_xor_cipher::break_single_byte_xor_cipher;

#[cfg(test)]

#[test]
pub fn test_fixed_xor() {
    use crate::set_1::challenge_1::hex_to_base64::{hex_to_base64, str_to_hex};
    let input = str_to_hex(&String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"));
    let output = String::from("Cooking MC's like a pound of bacon");
    assert_eq!(output.into_bytes(), break_single_byte_xor_cipher(&input));
}
