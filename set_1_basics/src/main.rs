//Convert hex to base64
//The string:
// 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
//Should produce:
//SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
//So go ahead and make that happen. You'll need to use this code for the rest of the exercises.

//Cryptopals Rule
//Always operate on raw bytes, never on encoded strings. Only use hex and base64 for pretty-printing.

//use std::string;

//use hex::FromHEx;
use base64::{Config, CharacterSet, encode_config};

fn main() {
    // Will  input the original hexadecimal string
    let s= "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("The original hexadecimal string: {}", s);
    
    //use the function we defined to convert from hex to based64
    let result= hex_to_base64(s.to_string());
    println!("Base64 encoded: {}", result)
}

fn hex_to_base64 (s: String)-> String {
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