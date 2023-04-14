
#![allow(unused)]

//use std::f32::consts::PI;
use std::{io, vec};
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::HashMap;
mod restaraunt;
use crate::restaraunt::order_food;


fn main(){

order_food();

}


