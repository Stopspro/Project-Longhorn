// in its final form, this should be a GNU Nano editor clone

#[macro_use]
extern crate horrorshow; // horrorshow for formatting with HTML
use horrorshow::prelude::*;
use horrorshow::doctype;


pub fn editor() {
    let mut file = File::create()?; 
    let linenumber = 1; // this is the line that we are on
    let line = String::from("{}", linenumber); 
    let key = String::new(); // this is the key that gets added to the string
    // HTML setup
    loop {
           
    }
}
