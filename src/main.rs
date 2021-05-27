// #![cfg_attr(not(test), no_std)]
// #![cfg_attr(not(test), no_main)]

#![no_std]
#![no_main]
#![feature(const_evaluatable_checked, const_generics)]
#![allow(non_snake_case, incomplete_features)]

use cosmian_std::prelude::scale_std::slice::Slice;
use cosmian_std::scale::{self, println, Channel, ClearModp, Reveal, SecretModp};
use cosmian_std::{prelude::*, OutputRow};

// use scale_std::bit_protocols::KOpL;
// use scale_std::fixed_point::*;
// use scale_std::integer::*;
// use scale_std::math::*;
// use scale_std::slice::Slice;
#[scale::main(KAPPA = 40)]
#[inline(always)]
fn main() {
    let test_1 = SecretInteger::<64>::from(10);
    let test_2 = SecretInteger::<64>::from(12);
    if i64::from(test_1.eq(test_2).reveal()) == 1 {
        println!("match");
    } else {
        println!("notmatch");
    }
    let test_1 = SecretInteger::<64>::from(10);
    let test_2 = SecretInteger::<64>::from(10);
    if i64::from(test_1.eq(test_2).reveal()) == 1 {
        println!("match");
    } else {
        println!("notmatch");
    }
    // let a = SecretModp::from(10);
    // let b = SecretModp::from(4343234);
    // a.test();
    // b.test();
    // let i_a = SecretInteger::<32>::from(a);
    // let i_b = SecretInteger::<32>::from(b);
    // let bit = i_a.eq(i_b);
    // let c_bit = bit.reveal();
    // let i_bit: i64 = i64::from(c_bit);
    // let mut slice = Slice::uninitialized(32);
    // for i in 0..32 {
    //     slice.set(i as u64, &SecretModp::from(1010101010));
    // }
    // let val = KOpL(|elt1, _elt2| elt1, slice.addr(0), slice.len() as i64);
    // println!("resp -- ", val.reveal());
    // if i_bit == 1 {
    //     println!("Equal");
    // } else {
    //     println!("Not Equal");
    // }
}
