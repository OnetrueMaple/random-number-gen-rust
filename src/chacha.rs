#![allow(unused)]

/*
    I'm just leaving a note here of a problem I'm noticing with my algorithm so far.
    This algorithm is requiring that I have data in a lot of different forms for specific kinds of changes.
        There has GOTTA be a way to keep the data more fundamental while deriving changes to speed this thing the fuck up.
        I'm honestly so irriated by watching my runtimes go up as I have to add ANOTHER datatype conversion before doing
        another fucking change to the data. If you ever come back to this... PLEASE address that. Moving on from this hurts me deep in my soul.
*/

pub fn chacha20_cipher(data: Vec<u8>, keys: [u32; 8], nonce: [u32; 3], counter: u32) -> Vec<u8> { // This function performs both encryption AND decryption of data. More below.
    let     length:     usize    = data.len();                                              // Length of data
    let mut encrypted:  Vec<u8> = Vec::with_capacity(length);                               // Stores an encrypted copy of the data. Built inside of the for-loops below.
    let mut keystream: Vec<u8> = Vec::with_capacity( 64usize * ((length / 64) + 1usize) );  // Stores the keystream (serialized output of chacha20_block()) used for encryption/decryption

    // For loops tick 'i' (or whatever you set the variable name to) upwards as they run. As 'i' ticks up I can reference each byte inside the 'temp' keystream.
    for i in 0..((length / 64) + 1usize) { // I have the loop set to stop creating keystream blocks (each 64 bytes, or 256 bits) once it matches or exceeds the length of our data
        let     temp: Vec<u8> = chacha20_block(keys, nonce, (counter + (i as u32)));    // Create a new keystream block
        for j in 0..64 { keystream.push(temp[j]) }                                      // Add each byte in the keystream block into the 'keystream' variable
    }

    // This loop goes through each byte in 'data' and 'keystream', and XOR's (exclusive OR) them together. The XOR function encrypts and decrypts, check the truth table below
    //      for an explanation.
    for i in 0..length { encrypted.push(data[i] ^ keystream[i]) };
    /*          -----------------------------
                |     XOR Truth Table       |
                |---------------------------|
                | Bit 1  | Bit 2  | Result  |
                | ------ | ------ | ------- |
                |   0    |   0    |   0     |
                |   0    |   1    |   1     |
                |   1    |   0    |   1     |
                |   1    |   1    |   0     |
                -----------------------------
            Let's see an example with 4 bits of data: 0101
            If I XOR my data '0101' with a specific set of bits, say: 0110...

            0101 ^                          0011
            0110    then I can take my      0110    I can get my original
            ----    result '0011' and       ----    data back!
            0011    XOR the bits again...   0101

        To put this in simpler terms, if a '1' is present in a bit column during XOR, it swaps the bit inside our data.
            So, it gets swapped back whenever we XOR again using the same bit sequence.
    */

    return encrypted;
}

// Allows chacha20_cipher() to encrypt arbitrary data... the datatype output (Vec<u8>) is the same as what's required inside chacha20_cipher's 'data' parameter: Vec<u8>
pub fn serialize_data(data: Vec<u32>) -> Vec<u8> { // This function turns a sequence of u32 numbers into u8 numbers instead.
    let mut serialized: Vec<u8> = Vec::new();   // New list of u8 numbers...
    let     length:     usize   = data.len();   // Length of our data...
    for i in 0..length {    // As 'i' goes up, references the next datapoint in our 'data'
        // For context, u32 numbers are represented like '0xYYYYYYYY', where Y is a character 0-9 and a-f. u8 is just 0xYY.
        // Also, bitwise AND (denoted '&') with a 1 (f here, for all bits, since f is 1111) returns the bits at that spot. 00 & 11 -> 00, 10 & 11 -> 10, ect.
        serialized.push( ((0xff000000u32 & data[i]) / 0x01000000u32) as u8 ); // Adds the two bits 0xYY000000 to the new list
        serialized.push( ((0x00ff0000u32 & data[i]) / 0x00010000u32) as u8 ); // Adds the two bits 0x00YY0000 to the new list
        serialized.push( ((0x0000ff00u32 & data[i]) / 0x00000100u32) as u8 ); // Adds the two bits 0x0000YY00 to the new list
        serialized.push(  (0x000000ffu32 & data[i])                  as u8 ); // Adds the two bits 0x000000YY to the new list
    }                                                                         //    where 'YY' represents the actual location of bits being read.

    return serialized
}

// Allows me to convert a string to the datatype I need for chacha20_cipher(), Vec<u8>
pub fn serialize_string(data: &str) -> Vec<u8> {
    let nums: Vec<u8> = data.as_bytes().to_vec();

    return nums
}

/* ---------------------------------------------------------------------------------------- */
// Actual code for encryption. Probably slower than it needs to be. PLEASE REVIST!!!
/* ---------------------------------------------------------------------------------------- */
fn reformat(data: u32) -> u32 {     // This function does what 'serialize_data' does, but it swaps the bit locations from high value to low value.
    let     temp:       [u8; 4] =   // An example is the number '90 AB CD EF' being reformatted turns into 'EF CD AB 90'
    [                               // You can see below that I swapped the order of ff's and 1s in the sequence compared to 'serialize_data'
        ( 0x000000ffu32 & data)                  as u8,
        ((0x0000ff00u32 & data) / 0x00000100u32) as u8,
        ((0x00ff0000u32 & data) / 0x00010000u32) as u8,
        ((0xff000000u32 & data) / 0x01000000u32) as u8
    ];

    let mut reformatted: u32     = 0u32; // Here, I have to add the numbers back into a new u32 number, where the digits are arranged accordingly.
    reformatted += (temp[3] as u32) * 0x00000001u32;
    reformatted += (temp[2] as u32) * 0x00000100u32;
    reformatted += (temp[1] as u32) * 0x00010000u32;
    reformatted += (temp[0] as u32) * 0x01000000u32;

    return reformatted
}

fn serialize_block(data: [u32; 16]) -> Vec<u8> { // I have to 'unformat' the numbers inside of chacha20_block after generating a keystream, where 'reformat' is called.
    let mut serialized: Vec<u8> = Vec::with_capacity(64); // Stores all the bytes inside of data in the correct order.

    for i in 0..16 {    // This didn't matter before, but the order of when I 'push' bytes into 'serialized' matters. The formatting change requires it be in THIS order.
        serialized.push(  (0x000000ffu32 & data[i])                  as u8 );
        serialized.push( ((0x0000ff00u32 & data[i]) / 0x00000100u32) as u8 );
        serialized.push( ((0x00ff0000u32 & data[i]) / 0x00010000u32) as u8 );
        serialized.push( ((0xff000000u32 & data[i]) / 0x01000000u32) as u8 );
    }

    return serialized
}

fn quarter_round(state: &mut[u32; 16], a: usize, b: usize, c: usize, d: usize) { // This is where my understanding falls apart a lot more, but if you have questions I'll try to explain
    state[a] = state[a].wrapping_add(state[b]); state[d] ^= state[a]; state[d] = state[d].rotate_left(16);  // what's happening at each point. A lot of it is just math that's hard to
    state[c] = state[c].wrapping_add(state[d]); state[b] ^= state[c]; state[b] = state[b].rotate_left(12);  // follow on purpose, though wrapping_add, ^= (XOR), and rotate_left (bit rotation)
    state[a] = state[a].wrapping_add(state[b]); state[d] ^= state[a]; state[d] = state[d].rotate_left( 8);  // are used because it doesn't destroy information passed to the function.
    state[c] = state[c].wrapping_add(state[d]); state[b] ^= state[c]; state[b] = state[b].rotate_left( 7);
}

fn chacha20_block(keys: [u32; 8], nonce: [u32; 3], counter: u32) -> Vec<u8> { // This function generates a keystream from data you specify. Keys, Nonces, Counter are all used
    let mut state: [u32; 16] =                                                // to regenerate the same keystream as long as those values remain the same across calls.
    [   // I'm just initializing it here. Tbh idk what these top 4 numbers do lmaooooo
        0x61707865, 0x3320646e, 0x79622d32, 0x6b206574,
        0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000
    ];

    // Set keys, nonces, and counter
    for i in 0..=7 { state[i + 4]  = reformat(keys[i])  }
    for i in 0..=2 { state[i + 13] = reformat(nonce[i]) }
    state[12] = counter;

    let initial: [u32; 16] = state; // I have to create a copy of the state before I mix the data up since I'm adding it back into the state later, as per
    for i in 1..=10 {               // https://datatracker.ietf.org/doc/html/rfc8439#section-2.3.2
        quarter_round(&mut state, 0,  4,  8, 12);
        quarter_round(&mut state, 1,  5,  9, 13);
        quarter_round(&mut state, 2,  6, 10, 14);
        quarter_round(&mut state, 3,  7, 11, 15);
        quarter_round(&mut state, 0,  5, 10, 15);
        quarter_round(&mut state, 1,  6, 11, 12);
        quarter_round(&mut state, 2,  7,  8, 13);
        quarter_round(&mut state, 3,  4,  9, 14);
    }

    // Add initial state back into block
    for i in 0..16 { state[i] = state[i].wrapping_add(initial[i]) }

    return serialize_block(state)
}

/*
// The code down here is KNOWN to work.
// I'd recommend not changing anything below this point. The code below should help you fix and verify the code above if you accidentally break it.

pub fn sanity_test() {
// These are the values used to make sure you can recover your encrypted data. Keys ~> Passwords, Nonce ~> Unique Identifiers (per cycle, ideally), counter grows with data size
    let     keys:          [u32; 8] = [0x00010203, 0x04050607, 0x08090a0b, 0x0c0d0e0f, 0x10111213, 0x14151617, 0x18191a1b, 0x1c1d1e1f];
    let     nonce:         [u32; 3] = [0x0, 0x4a, 0x0];
    let     counter:       u32      = 0x1;

    // This is the data we want to encrypt. The algorithm allows you to encrypt more than just text, but for simplicity I will encrypt just text.
    let     initial_str:   String   = "Ladies and Gentlemen of the class of '99: If I could offer you only one tip for the future, sunscreen would be it.".to_string();
    let     encrypted_str: Vec<u8>  = [
                                      0x6e, 0x2e, 0x35, 0x9a, 0x25, 0x68, 0xf9, 0x80, 0x41, 0xba, 0x07, 0x28, 0xdd, 0x0d, 0x69, 0x81,
                                      0xe9, 0x7e, 0x7a, 0xec, 0x1d, 0x43, 0x60, 0xc2, 0x0a, 0x27, 0xaf, 0xcc, 0xfd, 0x9f, 0xae, 0x0b,
                                      0xf9, 0x1b, 0x65, 0xc5, 0x52, 0x47, 0x33, 0xab, 0x8f, 0x59, 0x3d, 0xab, 0xcd, 0x62, 0xb3, 0x57,
                                      0x16, 0x39, 0xd6, 0x24, 0xe6, 0x51, 0x52, 0xab, 0x8f, 0x53, 0x0c, 0x35, 0x9f, 0x08, 0x61, 0xd8,
                                      0x07, 0xca, 0x0d, 0xbf, 0x50, 0x0d, 0x6a, 0x61, 0x56, 0xa3, 0x8e, 0x08, 0x8a, 0x22, 0xb6, 0x5e,
                                      0x52, 0xbc, 0x51, 0x4d, 0x16, 0xcc, 0xf8, 0x06, 0x81, 0x8c, 0xe9, 0x1a, 0xb7, 0x79, 0x37, 0x36,
                                      0x5a, 0xf9, 0x0b, 0xbf, 0x74, 0xa3, 0x5b, 0xe6, 0xb4, 0x0b, 0x8e, 0xed, 0xf2, 0x78, 0x5e, 0x42,
                                      0x87, 0x4d
                                      ].to_vec();

    let mut test_encryption: Vec<u8> = chacha20_cipher(serialize_string(&initial_str), keys, nonce, counter);
    if test_encryption.len() == encrypted_str.len() {
        for i in 0..(encrypted_str.len()) {
            if encrypted_str[i] != test_encryption[i] { println!("Bytes at index {} don't match! Expected {}, got {}!", i, encrypted_str[i], test_encryption[i]) }
        }
    } else { println!("The encrypted string and data from chacha20_cipher don't have the same length!") }
    test_encryption = chacha20_cipher(test_encryption, keys, nonce, counter);
    let mut s: String = String::new();

    if test_encryption.len() == encrypted_str.len() {
        let serialized_initial: Vec<u8> = serialize_string(&initial_str);
        for i in 0..(test_encryption.len()) {
            if serialized_initial[i] != test_encryption[i] { println!("Bytes at index {} don't match! Expected {}, got {}!", i, serialized_initial[i], test_encryption[i]) }
        }
        let decrypted_str: String = std::str::from_utf8(&test_encryption).expect("Decryption failed!").to_string();
        println!("Decrypted string:\n{:?}", decrypted_str);
    }
}

*/