use std::io;
use std::string::String;
use std::io::prelude::*;
use std::env;
use std::vec::Vec;
use std::thread;
use std::process;
use std::mem;
use rand::Rng;
extern crate rand;
fn lop(rng: u32) {
    let r = rng.clone();
    thread::spawn(move || roar(r));
    thread::sleep_ms(3);
}
fn roar(rng: u32) {
    let Em = [
        "１",
        "２",
        "３",
        "４",
        "５",
        "６",
        "８",
        "７",
        "９",
        "０",
        "　",
    ];

    let mut rng = rand::thread_rng().gen_range(1, 71);
    print!("{}{}", rng, Em[rng % 10]);
    match rng {
        rng if rng % 3 == 0 && rng % 2 == 0 => print!("{} ", Em[rng % 10]),
        rng if rng % 2 == 0 => print!("{}", rng),
        rng if rng % 3 == 0 => print!("{}", Em[rng % 10]),
        _ => print!("{}", Em[10]),
    }
}

fn main() {
    loop {
        lop(1);
    }
}
