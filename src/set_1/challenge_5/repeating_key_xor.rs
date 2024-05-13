

pub fn repeating_key_xor(input: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    let mut key_index = 0;
    let mut encrypted_input = Vec::new();
    let mut i = 0;
    for byte in input.iter(){
        key_index = i % key.len();
        i += 1;
        encrypted_input.push(byte ^ key[key_index]);
    }
    return encrypted_input;
}