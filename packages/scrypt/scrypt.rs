#![no_std]
extern crate cpuio;

use cpuio::Port;

pub fn editor() {
    let editorname = "scrypt v0.1";
    println!("{}", editorname);
    println!("the perfect text editor.")
    
    let linenumber = 1; // this is the line that we are on
    let line = String::from("{}", linenumber); 
    let key = String::new(); // this is the key that gets added to the string
    let mut keyboard: Port<u8> = unsafe { Port::new(0x60) };
    loop {
        if cmd = 1 { // new file on the block
            let line = String::from("~ ");
            let key = keyboard.read();
            let skip = 1;
        }
        
        if skip = 0 {
            if cmd = 2 { // open file
                
            }
        
            if skip = 0 {
                if cmd = 3 { // save file to block
                    let filename = String::from("")
                    let key = keyboard.read();
                }
                
                if cmd = 4 { // exit editor
                        break
                }
            }
        }    
    }
    println!("Exiting editor.")
}

pub fn editorspecific() {
    println!("scrypt v0.1");
    println!("the perfect text editor.")
    
    // load filename from command
    
    let linenumber = 1; // this is the line that we are on
    let line = String::from("{}", linenumber); 
    let mut keyboard: Port<u8> = unsafe { Port::new(0x60) };
    loop {
        
    }