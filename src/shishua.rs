#![allow(unused)]
// #[target_feature(enable = "avx2")] // Put this above a function, and if possible the code will run with AVX2 registers instead!

use std::arch::{simd::*};

#[target_feature(enable = "avx2")]
pub struct ShishuaState {
    state1: __m256i  = ,
    state2: __m256i  = ,
    output: __m256i  =
}