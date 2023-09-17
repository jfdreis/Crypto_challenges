//use std::string;

//use hex::FromHEx;
use base64::{Config, CharacterSet, encode_config};

fn main() {
    //Here is the solution for the first challenge of Set 1
    // Will  input the original hexadecimal string

    println!("Challenge 1 of Set 1");
    let s= "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("The original hexadecimal string: {}", s);
    
    //use the function we defined to convert from hex to based64
    let result= hex_to_base64(s.to_string());
    println!("Base64 encoded: {}", result);

    println!("Challenge 2 of Set 1");

    // Input
    let s= String::from("1c0111001f010100061a024b53535009181c");
    let t = String::from("686974207468652062756c6c277320657965");
    println!("We are going to do the XOR combination of {} and {}.",s,t);
    let ans= fixed_xor(s,t);
    println!("The solution to challenge 2 is {}",ans);
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


fn fixed_xor(s: String, t:  String) -> String {
    //convert hexadecimal to bytes
    let s_bytes = hex::decode(&s).expect("Invalid hexadecimal base input");
    let t_bytes = hex::decode(&t).expect("Invalid hexadecimal base input");
    
    // Perform the XOR operation on each pair of bytes
    let xor_result: Vec<u8> = s_bytes.iter().zip(&t_bytes).map(|(a, b)| a ^ b).collect();    
    
    // Convert the result back to a hexadecimal string
    let xor_hex: String = xor_result.iter().map(|byte| format!("{:02x}", byte)).collect();
    xor_hex
   
}