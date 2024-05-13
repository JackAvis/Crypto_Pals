use crate::set_1::challenge_4::detect_single_character_xor::{detect_single_character_xor, text_file_to_string};

use std::process::Command;


#[cfg(test)]

#[test]
pub fn test_detect_single_character_xor() {
    use crate::set_1::challenge_1::hex_to_base64::str_to_hex;

    let file_path = String::from("./src/set_1/challenge_4/4.txt");
    let file_content= text_file_to_string(&file_path);
    let output = String::from("Now that the party is jumping\n");
    assert_eq!(output.into_bytes(), detect_single_character_xor(&file_content));
}
