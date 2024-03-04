

pub fn repeating_key_xor(input: &String, key: &String) -> String {
    let input_bytes = input.as_bytes();
    let key_bytes = key.as_bytes();
    let mut key_index = 0;
    let mut encrypted_input = Vec::new();
    let mut i = 0;
    for byte in input_bytes.iter(){
        key_index = i % key_bytes.len();
        i += 1;
        encrypted_input.push(byte ^ key_bytes[key_index]);
    }
    return hex::encode(&encrypted_input);
}