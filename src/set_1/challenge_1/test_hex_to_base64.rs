use crate::set_1::challenge_1::hex_to_base64::hex_to_base64;

#[cfg(test)]

#[test]
pub fn test_hex_to_base64() {
    let hex_value: String = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let base_64_of_hex: String = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    
    assert_eq!(base_64_of_hex, hex_to_base64(&hex_value.into_bytes()));
}
