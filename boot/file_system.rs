#![no_std]
extern crate postgres;
extern crate cpuio;

use std::fs;

pub fn scan() { // runs a scan of all the files/packages installed.
  loop {
    let dir =
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
      println!("File: {}", path.unwrap().path().display())
  
    println!("DIR: { }", dir);
  }
}

pub fn virus() { // by default, the virus option should come disabled. By using the config though, it should be activated.
  loop {
    let paths = fs::read_dir(dir).unwrap();
  }
}

pub fn update() { // updates the file
  
}
  
pub fn block_encryption() {
    
}
  
  
