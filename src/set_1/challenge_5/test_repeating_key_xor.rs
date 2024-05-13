use crate::set_1::challenge_4::detect_single_character_xor::{detect_single_character_xor, text_file_to_string};

use std::process::Command;


#[cfg(test)]

#[test]
pub fn test_repeating_key_xor() {
    use crate::set_1::{challenge_1::hex_to_base64::from_hex, challenge_5::repeating_key_xor::repeating_key_xor};
    let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".as_bytes().to_vec();
    let key = "ICE".as_bytes().to_vec();
    let output = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".as_bytes().to_vec();
    let output_decoded= from_hex(&output);
    assert_eq!(output_decoded, repeating_key_xor(&input, &key));
}
