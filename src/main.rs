#![allow(unused)]

use std::io;
use rand:: Rng;
use std::io:: {Write, BufReader, BufRead, ErrorKind};
mod set_1;
use set_1::challenge_1::hex_to_base64::hex_to_base64;

fn main() {
    let hex_value: String = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    hex_to_base64(&hex_value);
}
