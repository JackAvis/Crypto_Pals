use core::num;
use std::{cmp::min, f32::INFINITY};

pub fn hamming_distance(input: &Vec<u8>, compare: &Vec<u8>) -> u32 {
    let mut total_diff = 0;
    for (i, byte) in input.iter().enumerate(){
        total_diff += (byte ^ compare[i]).count_ones();
    }
    return total_diff;
}


pub fn break_repeating_key_xor(input: &Vec<u8>) -> Vec<u8>{

    let keysize = find_keysize_repeating_key_xor(&input);
    println!("{}", keysize);
    let mut blocks: Vec<u128> = Vec::new();
    let mut counter = 0;
    let mut block = 0;
    for byte in input.iter(){
        if counter == keysize{
            blocks.push(block);
            block = 0;
            counter = 0;
        }
        block = block + (*byte as u128) << counter*8;
        counter += 1;
    }

    let mut transposed_blocks: Vec<u32> = Vec::new();
    let mut num_bytes_left = keysize;

    return Vec::new();
}


pub fn find_keysize_repeating_key_xor(input: &Vec<u8>) -> u32 {
    let mut min_dist = 40*9;
    let mut keysize = 0;
    for keysize_guess in  2..=40{
        let mut first_block = Vec::new();
        for i in 0..=keysize_guess{
            first_block.push(input[i as usize])
        }
        let mut second_block = Vec::new();
        for j in keysize_guess..=keysize_guess*2{
            second_block.push(input[j as usize])
        }
        let distance = hamming_distance(&first_block, &second_block);
        let normalized_distance = distance / keysize_guess;
        if (normalized_distance < min_dist){
            min_dist = normalized_distance;
            keysize = keysize_guess;
        }
    }
    return keysize;

}