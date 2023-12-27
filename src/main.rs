#![allow(unused)]

use std::io;
use std::io::{Write, BufRead, BufReader,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("what is name");
    let mut name:String = String::new();
    let greeting:&str="Nice to meet you";
    io::stdin().read_line(&mut name)
        .expect("Failed to read line");
    println!("Hello, {}! {}",name.trim_end() ,greeting);
} 