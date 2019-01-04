extern crate socool_parser;
extern crate weresocool;
extern crate num_rational;
use socool_parser::ast::Op::*;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use num_rational::Rational64;

fn main() {
    println!("\nHello Danny's WereSoCool Scratch Pad");

    let s = Sequence { operations: vec![
        TransposeM { m: Rational64::new(2,1)},
    ]};
    let o = Overlay { operations: vec![
        TransposeM { m: Rational64::new(2,1)}
    ]};

    let hash = calculate_hash(&AsIs);
    let hash2 = calculate_hash(&o);
    let hash3 = calculate_hash(&s);
    println!("{:?}", hash);
    println!("{:?}", hash2);
    println!("{:?}", hash3);
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
