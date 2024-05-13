use crate::set_1::challenge_2::fixed_xor::fixed_xor;

#[cfg(test)]

#[test]
pub fn test_fixed_xor() {
    use crate::set_1::challenge_1::hex_to_base64::from_hex;


    let input: Vec<u8> = String::from("1c0111001f010100061a024b53535009181c").into_bytes();
    let hex_decoded  = from_hex(&input);
    let xor_val: Vec<u8> = String::from("686974207468652062756c6c277320657965").into_bytes();
    let xor_decoded = from_hex(&xor_val);
    let output: Vec<u8> = String::from("746865206b696420646f6e277420706c6179").into_bytes();
    let output_hex = from_hex(&output);
    assert_eq!(output_hex, fixed_xor(&hex_decoded, &xor_decoded));
}
