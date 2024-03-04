#![allow(unused)]

use std::io;
use rand:: Rng;
use std::io:: {Write, BufReader, BufRead, ErrorKind};
mod set_1;
use set_1::challenge_1::hex_to_base64::hex_to_base64;

fn main() {
    let hex_value: String = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    hex_to_base64(&hex_value);
}
