
#![allow(unused)]

use std::{io, vec};
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main(){

let vec1: Vec<i32> = Vec::new();

let mut vec2:Vec<i32> = vec![1,2,3,4];

vec2.push(5);

println!("1st: {}", vec2[0]);

let second = &vec2[1];


match vec2.get(1) {
    Some(second) => println!("2nd: {}", second),
    None => println!("No second value"),
}

assert_eq!(None, vec2.get(0..6));

for i in &mut vec2{
    *i *=2;
}

for i in &vec2{
    println!("{}", i);
}
println!("Vec Length {}", vec2.len());
println!("Pop: {:?}", vec2.pop());



}


