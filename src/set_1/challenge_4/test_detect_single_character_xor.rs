use crate::set_1::challenge_4::detect_single_character_xor::{detect_single_character_xor, text_file_to_string};

use std::process::Command;


#[cfg(test)]

#[test]
pub fn test_detect_single_character_xor() {
    let file_path: String = String::from("./src/set_1/challenge_4/4.txt");
    let file_content: String = text_file_to_string(&file_path);
    let output: String = String::from("Now that the party is jumping\n");
    assert_eq!(String::from(output), detect_single_character_xor(&file_content));
}
