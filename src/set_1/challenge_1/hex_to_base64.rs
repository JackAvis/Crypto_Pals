extern crate base64;
use std::{ptr::null, u8};
use base64::{Engine as _, engine::general_purpose};
use hex;
pub fn hex_to_base64(hex: &String) -> String{
    let bytes: Vec<u8> = hex::decode(hex).unwrap();
    let base64: String = general_purpose::STANDARD.encode(&bytes);
    return base64;
}


