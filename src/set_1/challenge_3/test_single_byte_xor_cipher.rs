use crate::set_1::challenge_3::single_byte_xor_cipher::single_byte_xor_cipher;

#[cfg(test)]

#[test]
pub fn test_fixed_xor() {
    let input: String = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let output: String = String::from("746865206b696420646f6e277420706c6179");
    assert_eq!(String::from(output), single_byte_xor_cipher(&input));
}
