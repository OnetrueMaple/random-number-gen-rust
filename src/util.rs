use std::fs::File;
use std::io::Read;

pub fn random(bound: u32) -> u32 { // 'pub' means public (access outside file), 'fn' means function declaration, 'bound: u8' var_name: datatype, '-> u8' means returns u8 datatype
    let mut f                = File::open("/dev/urandom").expect("The file could not be opened!"); // Have to explicitly handle the possibility of failing to open the file
    let limit:      u32      = u32::MAX - (u32::MAX % bound);
    let mut bytes:  [u8; 4]  = [0u8; 4];
    f.read_exact(&mut bytes).expect("Could not read bytes from urandom!"); // Stores 1 byte to turn into a random number, u8. The number of bytes stored is determined by the size of array.
    loop {                                                                 // Forever loop, exits only when returned or broken out of.
        let mut temp: u32 = 0;
        //println!("Bytes: {:?}", bytes);
        temp += ((bytes[0] as u32) << 24u32) +
                ((bytes[1] as u32) << 16u32) +
                ((bytes[2] as u32) << 8u32)  +
                  bytes[3] as u32;
        //println!("Candidate number: '{}'", temp);
        if temp < limit { return temp % bound; }
        f.read_exact(&mut bytes).expect("Could not read bytes from urandom!");
    }
}

pub fn return_timestamp() -> u128 {
    return std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("Holy negative time...????").as_micros();
}