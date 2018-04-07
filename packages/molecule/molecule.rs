extern crate cpuio;

use cpuio::Port;

pub fn editor() {
    println!("                                   ________    _______                      ________ 
|          |   | | | |   |        |           |         |     |  |         |
| |      | |   |     |   |        |________   |         |     |  |         |________
|  |   |   |   |     |   |        |           |         |     |  |         |      
|    |     |   | | | |   |______  |________   |_______  | | | |  |______   |________  "); // do not edit formatting, it has been tested to work like this
    
    let linenumber = 1; // this is the line that we are on
    let line = String::from("{}", linenumber); 
    let key = String::new(); // this is the key that gets added to the string
    let mut keyboard: Port<u8> = unsafe { Port::new(0x60) };
    loop {
        if cmd = 1 { // new file on the block
            let command = keyboard.read();
            let key = keyboard.read();
        }
        
        if cmd = 2 { // open file
            let command = keyboard.read();
            let key = keyboard.read();
        }
        
        if cmd = 3 { // save file to block
            let command = keyboard.read();
            let key = keyboard.read();
        }
        
        if cmd = 4 { // exit editor
            
        }
    }
}
