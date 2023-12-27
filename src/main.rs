#![allow(unused)]

use std::io;
use std::io::{Write, BufRead, BufReader,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// fn main() {
//     println!("what is name");
//     let mut name:String = String::new();
//     let greeting:&str="Nice to meet you";
//     io::stdin().read_line(&mut name)
//         .expect("Failed to read line");
//     println!("Hello, {}! {}",name.trim_end() ,greeting);
// } 


fn main() {
   const ONE_MIL:u32 = 1_000_000;
   const PI:f32 = 3.141592;
   let age: &str = "47";
   let mut age :u32 = age.trim().parse()
       .expect("Age wasn't assigned a number");
   age = age + 1;
   println!("I'm {} ND I wanr ${}", age, ONE_MIL);
    
}