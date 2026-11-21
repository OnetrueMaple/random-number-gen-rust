#![allow(unused)]

mod chacha;
use chacha::{serialize_data, serialize_string, chacha20_cipher};

mod util;
use util::{random, return_timestamp};

// random seed: [86363528, 723882832, 2262373003, 2262601938, 1944573353, 772361550, 3586108275, 3875139511, 2674865132, 4225279034, 2079719790, 988947929, 1403664618, 870533555, 2321778250, 3054219076]

fn main() {
    let     micro_units: f32       = 0.000001;              // Using this to multiply by the runtime measurement at the end of the program. It's in whole numbers...
    let     start:       u128      = return_timestamp();    // Start time of the program.

    let     rand_seed:   [u32; 16] =
    [
        0x0525CD88, 0x2B259350, 0x86D9128B, 0x86DC90D2,
        0x73E7D5A9, 0x2E094D4E, 0xD5BFAB73, 0xE6F9EFB7,
        0x9F6F33EC, 0xFBD8A43A, 0x7BF6016E, 0x3AF225D9,
        0x53AA38EA, 0x33E349B3, 0x8A63864A, 0xB60BAF44
    ];



    let end   = return_timestamp() - start;
    println!("Elapsed Time: {}s", (end as f32 * micro_units));
}