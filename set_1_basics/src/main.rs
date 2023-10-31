extern crate base64;
use base64::decode;


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
            let mut key_v: Vec<u8> =vec![];
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
    let s="this is a test".as_bytes();
    let t="wokka wokka!!!".as_bytes();
    let vec_s=Vec::from(s);
    let vec_t=Vec::from(t);
    let d=hamming_distance(&vec_s,&vec_t);
    if d==37 {
        println!("The computation of the hamming distance is working well")
    }
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
    //println!("The text is {:?}",text);  
    //println!("......");
    let cleaned_text = text.replace("\r", "").replace("\n", "");
    //println!("The text is {:?}", cleaned_text);

    let base64_string = &cleaned_text; 
    let mut text:Vec<u8>=vec![];
    if let Ok(bytes) = decode(base64_string) {
        //println!("Decoded bytes: {:?}", bytes);
        text=bytes;
    } else {
        println!("Failed to decode Base64 string.");
    }
    //println!("The text in bytes is {:?}", text);

 
     //For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second KEYSIZE worth of bytes, and find the edit distance between them.
    //Normalize this result by dividing by KEYSIZE.

    let mut avg_hamming_distances: Vec<(usize,f64)> =vec![];
    for keysize in 2..41 {
        let mut hamming_distance_sum = 0;
        let mut chunk_count = 0;

        for chunk in text.chunks(keysize) {
            if chunk.len() == keysize {
                if let Some(next_chunk) = text.get((chunk_count+1) * keysize..(chunk_count + 2) * keysize) {
                    let chunk_vec: Vec<u8> = chunk.to_vec();
                    let next_chunk_vec: Vec<u8> = next_chunk.to_vec();
                    //println!("chunk {:?}",chunk);
                    //println!("next chunk {:?}",next_chunk);

                    hamming_distance_sum += hamming_distance(&chunk_vec, &next_chunk_vec);
                    chunk_count += 1;
                }
            }
        }
        let avg_distance = hamming_distance_sum as f64 / (chunk_count as f64 * keysize as f64);
        avg_hamming_distances.push((keysize, avg_distance));
    }

    avg_hamming_distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let best_keysize = avg_hamming_distances[0].0;

    println!("The probable key size is: {}", best_keysize);
    //println!("{:?}",avg_hamming_distances);
    //Now I am going to take the ciphered text and divide it in blocks of lenght keysize and transpose those block  
   
   //Number of blocks of lenght keysize:
   let n_blocks= text.len()/best_keysize;
   //length of the remainder part of the block that may have size less than keysize.
   
   //println!("Number of blocks {}. \nLength of the remainder {}.",n_blocks, remainder);
   let mut text_iterator= text.iter(); //creating iterator
   let mut text_per_block: Vec<Vec<u8>> =vec![];
    for i in 0..(n_blocks+1) { // with n_block+1 we make sure the remainder will be the last entry in the vector with blocks.
        let mut  a=Vec::new();
        for _ in (i*best_keysize)..((i+1)*best_keysize){
            if let Some(ch) =text_iterator.next()  {
                a.push(*ch);
            } else {
                //println!("No more characters");
                break
            }
        }
        text_per_block.push(a);
    }
    //println!("The original text in bytes {:?}",text);
    //println!("The text per blocks {:?}",text_per_block);

     //Now the aim is to get the transpose of this.
     let mut text_per_block_transpose: Vec<Vec<u8>> = vec![];
     for j in 0..text_per_block[0].len(){ // "collumns index" 
         let mut a: Vec<u8>=Vec::new();
         if j<5{
            for i in 0..text_per_block.len(){ // "rows index"
                let ch=text_per_block[i][j];
                //println!("This should be changing {:?}",ch);
                a.push(ch);
            }
        } else {
            for i in 0..text_per_block.len()-1{ // "rows index"
                let ch=text_per_block[i][j];
                //println!("This should be changing {:?}",ch);
                a.push(ch);
            }
        }
        text_per_block_transpose.push(a);
     }
     //println!("Text per blocks transposed {:?}", text_per_block_transpose);
     //println!("{}",text_per_block_transpose.len());

     //At this point we have the text_per_blocks_transpose, we want to find the key for each vector. LEt us use challenge 3.
    let all_chars=all_chars_in_u8();

    //We are assuming each row in text_per_blocks_transpose was XOR against a caracther. Hence we will XOR it against characters.
    //If after XORing we get a high percentage of letters, we save that character
    let mut possibilities:Vec<u8> =vec![];
    for i in 0..text_per_block_transpose.len(){ //row index
        for &j in &all_chars {
            let mut key_v: Vec<u8> =vec![j];
            while key_v.len() < text_per_block_transpose[i].len() {//making the key as long as the text to transpose
                key_v.push(j);
            };
            let xor_result: Vec<u8> = text_per_block_transpose[i].iter().zip(&key_v).map(|(a, b)| a ^ b).collect();    // 
            let tax=percentage_letters_in_text(xor_result);
            if tax > 0.8 {
                possibilities.push(j);
            }
        }
        //println!("The possible key(s) for row {} is(are) {:?}",i,possibilities);
    }
    // We have the probable key

    //println!("The probable key is {:?}", possibilities);
    //println!("The text we want to decipher written in bytes is: \n {:?}",text);
    let m=text.len();
    let key = cyclic_repeat_bytes(&possibilities, m);
    let xor_result: Vec<u8> = text.iter().zip(&key).map(|(a, b)| a ^ b).collect();    // 
    //println!("{:?}",xor_result);
    let decoded:String  = String::from_utf8(xor_result.clone()).unwrap();
    println!("The decoded text is");
    println!("{}",decoded);

}   