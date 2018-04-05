#![no_std]

extern crate postgres;

use postgres::{Connection, SslMode};
use std::fs::File;
use std::io::prelude::*;

pub fn download() {
  
}

pub fn list() {
  let mut file = File::open("docs.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  println!("{}", contents)
}
