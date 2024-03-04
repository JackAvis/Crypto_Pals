use crate::set_1::challenge_2::fixed_xor::fixed_xor;

#[cfg(test)]

#[test]
pub fn test_fixed_xor() {
    let input: String = String::from("1c0111001f010100061a024b53535009181c");
    let xor_val: String = String::from("686974207468652062756c6c277320657965");
    let output: String = String::from("746865206b696420646f6e277420706c6179");
    assert_eq!(String::from(output), fixed_xor(&input, &xor_val));
}
