// just a Vim clone
#![no_std]
extern crate cpuio;
// extern crate longhorn-essentials; WIP

mod vga_buffer;

use cpuio::Port;

pub fn editor() {
    let editorname = "scrypt v0.1";
    println!("{}", editorname);
    println!("the perfect text editor.")
    println!("")
    
    let linenumber = 1; // this is the line that we are on
    let line = String::from("{}", linenumber); 
    let key = String::new(); // this is the key that gets added to the string
    // this is the scrypt console, not the editing bit
    loop {
        if cmd = 1 { // new file on the block
            let line = String::new();
            let mut filename = String::new();
            // longhorn-essentials::Ports::PortSetup();
            
            if backspace = 1 {
            
            }
            
            if charnum = 
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
    println!("")
    
    // load filename from command
    
    let linenumber = 1; // this is the line that we are on
    let line = String::from("{}", linenumber); 
    loop {
        
    }
