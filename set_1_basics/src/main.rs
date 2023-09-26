use set_1_basics::*;

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
    println!("The solution in ascii values is {}", hex_to_ascii(&ans));

    println!("Challenge 3 of Set 1");

    //string that I want to decipher
    let s="1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    
    //I am going to make a vector whose entries are u8 values corresponding to space or letters
    let mut letters: Vec<u8> =vec![32]; // it as the key for the space

    for letter in 65..=90 {     //adding the keys for uppercase letters
        letters.push(letter);
    }
    for letter in 97..=122 {     //adding the keys for lowercase letters
        letters.push(letter);
    }

    //Now as the  string that  we want to decipher was xor against a caracther we will see what happens when we xor "s" agains letters.
    //If after XORing we get a high percentage of letter, we save that character
    let mut possibilities:Vec<u8> =vec![];
    for i in letters {
        let mut key_v: Vec<u8> =vec![i];
        while key_v.len() < s.len()/2 {
            key_v.push(i);
        };
        
        let t= xor_hexa_bytes_get_bytes(s,&key_v);
        let tax=percentage_letters_in_text(t);
        if tax > 0.7 {
            possibilities.push(i);
        }
    }
    println!("The possible key(s) is(are) {:?}",possibilities);

    // Here we print everything that we obtain while XORing against the entries in possibilities
    for i in possibilities {
        let mut key_v: Vec<u8> =vec![i];
        while key_v.len() < s.len()/2 {
            key_v.push(i);
        };
        let u= xor_hexa_bytes_get_string(s,&key_v);
        println!("For key {} the result is result: {:?}", i, u);
    }
}
    

    






