use set_1_basics::*;
use std::fs;

fn main() {
    //Here is the solution for the first challenge of Set 1
    // Will  input the original hexadecimal string

    println!("Challenge 1 of Set 1");
    let s= "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("The original hexadecimal string: {}", s);    
    //use the function we defined to convert from hex to based64
    let result= hex_to_base64(s);
    println!("Base64 encoded: {}", result);
    // convert to ascii
    let s_ascii=    hex_to_ascii(s);
    println!("The original string in ascii: {}",s_ascii);

    println!("Challenge 2 of Set 1");

    // Input
    let s ="1c0111001f010100061a024b53535009181c";
    let t = "686974207468652062756c6c277320657965";
    println!("We are going to do the XOR combination of {} and {}.",s,t);
    println!("The original strings in ascii values: {} AND {}", hex_to_ascii(s),hex_to_ascii(t));
    let ans= fixed_xor(s,t);
    println!("The solution to challenge 2 is {}",ans);
    println!("The solution in ascii values is '{}'.", hex_to_ascii(&ans));

    println!("Challenge 3 of Set 1");

    //string that I want to decipher
    let s="1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    
    //Here I make a vector use entries are characters.e
    let all_chars=all_chars_in_u8();

    //As the  string to decipher was XOR against a caracther we will XOR it against characters.
    //If after XORing we get a high percentage of letters, we save that character
    let mut possibilities:Vec<u8> =vec![];
    for &i in &all_chars {
        let mut key_v: Vec<u8> =vec![i];
        while key_v.len() < s.len()/2 {
            key_v.push(i);
        };       
        let t= xor_hexa_bytes_get_bytes(s,&key_v);
        let tax=percentage_letters_in_text(t);
        if tax > 0.8 {
            possibilities.push(i);
        }
    }
    println!("The possible key(s) is(are) {:?}",possibilities);
    let mut key_v: Vec<u8>=vec![];
    // Here we have everything that we obtain while XORing against the entries in possibilities
    for i in possibilities {
        while key_v.len() < s.len()/2 {
            key_v.push(i);
        };
        if let Some(u) = xor_hexa_bytes_get_string(s, &key_v) {
            println!("Solution for Challenge 3");
            println!("For key {} the result is: {:?}", i, u);
        } else {
            println!("Solution for Challenge 3");
            println!("For key {} the result is not a valid UTF-8 string.", i);
        }
        key_v=vec![];
    }


    println!("Challenge 4 of Set 1");

    let file_path = "src/list_to_decipher.txt";
    let text= match fs::read_to_string(file_path) {
        Ok(contents) => {
            // Successfully read the file, 'contents' contains the file's content as a string
            contents
        }
        Err(e) => {
            // Handle the error, e.g., file not found, permissions issue, etc.
            println!("Error reading the file: {}", e);
            String::new()
        }
    };
    
    for line in text.lines(){
        let mut possibilities:Vec<u8> =vec![];
        // in the next for I look for possible keys.
        // as before, the idea is:
        // if after XORing with a certain key I get a high percentage of letters,
        // then we save those keys.
        for &i in &all_chars {
            let mut key_v: Vec<u8> =vec![i];
            while key_v.len() < line.len()/2 {
                key_v.push(i);
            };
            let t= xor_hexa_bytes_get_bytes(line,&key_v);
            let tax=percentage_letters_in_text(t);
            if tax > 0.8 {
                possibilities.push(i);
            }
        }
        //if there are no possibilites I do nothing.
        if possibilities.len() !=0 {
            let mut key_v: Vec<u8>=vec![];
            for i in possibilities {
                while key_v.len() < line.len()/2 {
                    key_v.push(i);
                };
                let decoded = match xor_hexa_bytes_get_string(line, &key_v) {
                    Some(u) => u,
                    None => continue 
                };
                key_v=vec![];
                println!("The decoded string of line {:?}  with key {:?} is: \n {:?}",line,i,decoded);
            }
        }
        //else {
        //    println!("We found no possible keys");
        //}
    }
    
    println!("Challenge 5 of Set 1");

    //Input
    let s="Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    println!("{s}");
    let s_hex= string_to_hex(s);
    let mut key=string_to_hex("ICE");
    let m=s_hex.len();
    key=cyclic_repeat(&key, m);
    let ans=fixed_xor(&s_hex, &key);
    println!("{ans}");
    if ans=="0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"{
        println!("right answer");
    }

    println!("Challenge 6 of Set 1");

    //computing Hamming distance. Checking it is working well
    let s=string_to_hex("this is a test");
    let t=string_to_hex("wokka wokka!!!");
    let d=hamming_distance(&s, &t);
    println!("{d}");

    let file_path = "src/list_challenge_6.txt";
    let text= match fs::read_to_string(file_path) {
        Ok(contents) => {
            // Successfully read the file, 'contents' contains the file's content as a string
            contents
        }
        Err(e) => {
            // Handle the error, e.g., file not found, permissions issue, etc.
            println!("Error reading the file: {}", e);
            String::new()
        }
    };
    let text_hex=string_to_hex(&text);
    
    // I will try  to do this:
    //For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second KEYSIZE worth of bytes, and find the edit distance between them.
    //Normalize this result by dividing by KEYSIZE.
}

    






