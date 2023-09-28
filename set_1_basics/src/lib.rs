use base64::{Config, CharacterSet, encode_config};
use std::str;


pub fn hex_to_base64 (s: &str)-> String { //want &str because I do not need ownership of the string
    //Convert this string to bytes.
    //Note that decode returns a a Result<Vec<u>> we need to take handle the error case.
    let bytes = match hex::decode(s){
        Ok(bytes) => bytes,
        Err(_) => {
            let a = String::from("Error decoding hexadecimal string: {}");
            return a;
        }
    };
    //Convert from bytes to base64
    let s_in_base64 = encode_config(&bytes, Config::new(CharacterSet::Standard, false));
    String::from(s_in_base64)
}

pub fn fixed_xor(s: &str, t:  &str) -> String {
    //convert hexadecimal to bytes
    let s_bytes = hex::decode(&s).expect("Invalid hexadecimal base input");
    let t_bytes = hex::decode(&t).expect("Invalid hexadecimal base input");
    
    // Perform the XOR operation on each pair of bytes
    let xor_result: Vec<u8> = s_bytes.iter().zip(&t_bytes).map(|(a, b)| a ^ b).collect();    
    
    // Convert the result back to a hexadecimal string
    let xor_hex: String = xor_result.iter().map(|byte| format!("{:02x}", byte)).collect();
    xor_hex
   
}

pub fn hex_to_ascii(s: &str) -> String {
    let mut ascii_values = String::new();
    let mut hex_buffer = String::new(); // Store two hexadecimal characters.

    for hex_char in s.chars() {
        hex_buffer.push(hex_char);

        // When we have two hexadecimal characters in the buffer, parse and convert to ASCII.
        if hex_buffer.len() == 2 {
            //println!("{hex_buffer}");
            if let Ok(byte) = u8::from_str_radix(&hex_buffer, 16) {
                ascii_values.push(byte as char);
            } else {
                return String::from("Error converting hexadecimal to ASCII values");
            }

            hex_buffer.clear(); // Clear the buffer for the next pair.
        }
    }

    ascii_values
}

pub fn xor_hexa_bytes_get_bytes(s: &str, v: &Vec<u8>)-> Vec<u8>{
    let s_bytes = hex::decode(&s).expect("Invalid hexadecimal base input");
    let ans: Vec<u8> = s_bytes.iter().zip(v).map(|(a, b)| a ^ b).collect();
    ans
}

pub fn xor_hexa_bytes_get_string(s: &str, v: &Vec<u8>)-> Option<String>{
    let s_bytes = hex::decode(&s).expect("Invalid hexadecimal base input");
    let ans: Vec<u8> = s_bytes.iter().zip(v).map(|(a, b)| a ^ b).collect();
    if let Ok(utf8_string) = String::from_utf8(ans.clone()) {
        Some(utf8_string)
    } else {
        None
    }
}


//This function get a string in hexadecimal and determines the percentage
// of the symbols that correspond to lowercase letter and space

pub fn percentage_letters_in_text(v: Vec<u8>)-> f64 {
    let mut letters: Vec<u8> =vec![32]; // it as the key for the space

    // we do not add the keys for uppercase letters because most letters in a sentence are lowercase
    //we add lowercase letters

    for letter in 97..=122 {
        letters.push(letter);
    }

    let mut a=0.0;
    for value in &v {
        if letters.contains(value) {
            a = a + 1.0;
        }
    }
    a/v.len() as f64
}


//The next function return a vector will all the character writen as u8
pub fn all_chars_in_u8()->Vec<u8>{
    let mut all_chars: Vec<u8>=vec![];
    for c in 32..=126{
        all_chars.push(c);
    }
    for c in 128..=255{
        all_chars.push(c);
    }
    all_chars
}
//letters  and space in u8
pub fn letters_space_in_u8()->Vec<u8>{
    let mut letters: Vec<u8> =vec![32]; // it as the key for the space

    for letter in 65..=90 {     //adding the keys for uppercase letters
        letters.push(letter);
    }
    for letter in 97..=122 {     //adding the keys for lowercase letters
        letters.push(letter);
    }
    letters
}