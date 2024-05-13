extern crate base64;
use std::{ptr::null, u8};
use base64::{alphabet::STANDARD, engine::general_purpose, Engine as _};
use hex;
use str;

pub fn from_hex(input:&Vec<u8>) -> Vec<u8>{
    return hex::decode(&input).unwrap();
}

pub fn to_hex(input:&Vec<u8>) -> String{
    return hex::encode(&input);
}

pub fn to_base64(input:&Vec<u8>) -> String{
    return general_purpose::STANDARD.encode(&input);
}

pub fn from_base64(input:&Vec<u8>) -> Vec<u8>{
    return general_purpose::STANDARD.decode(&input).unwrap();
}

pub fn hex_to_base64(input:&Vec<u8>) -> String {
    return to_base64(&from_hex(&input));
}

pub fn str_to_hex(input: &String) -> Vec<u8> {
    return from_hex(&input.clone().into_bytes());
}

