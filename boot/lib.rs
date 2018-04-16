#![feature(lang_items)]
#![feature(const_fn)]
#![feature(const_unique_new)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;
#[macro_use]
extern crate bitflags;
extern crate x86_64;
extern crate cpuio;

use cpuio::Port;

#[macro_use]
mod vga_buffer;
mod memory;
mod config;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
    use memory::FrameAllocator;

    vga_buffer::clear_screen();
    config::shellscript();
    config::scryptconfig();
        
    }
    // terminal time
    println!("Project Longhorn, {}", version);
    let mut keyboard: Port<u8> = unsafe { Port::new(0x60) };
    let command = String::new();
    let scancode = keyboard.read();
    loop {
        if caps = 0 {
            match scancode {
    	        "1C" => let letter = "a"; // make code
	    	    "F01C" => let letter = "a"; // break code 
	            "32" => let letter = "b"; // repeat
    		    "F032" => let letter = "b"; 
	            "21" => let letter = "c";
    		    "F021" => let letter = "c";
	            "23" => let letter = "d";
    		    "F023" => let letter = "d";
	            "24" => let letter = "e";
    		    "F024" => let letter = "e";
	            "2B" => let letter = "f";
	    	    "F02B" => let letter = "f";
	            "" => let letter = "g";
	    	    " " => let letter = "g";
	            " " => let letter = "h";
	    	    " " => let letter = "h";
	            " " => let letter = "i";
	    	    " " => let letter = "i";
	            " " => let letter = "j";
	            " " => let letter = "j";
	            " " => let letter = "k";
		        " " => let letter = "k";
	            " " => let letter = "l";
		        " " => let letter = "l";
	            " " => let letter = "m";
		        " " => let letter = "m";
	            " " => let letter = "n";
		        " " => let letter = "n";
	            " " => let letter = "o";
		        " " => let letter = "o";
	            " " => let letter = "p";
		        " " => let letter = "p";
	            " " => let letter = "q";
		        " " => let letter = "q";
	            " " => let letter = "r";
		        " " => let letter = "r";
	            " " => let letter = "s";
		        " " => let letter = "s";
	            " " => let letter = "t";
		        " " => let letter = "t";
	            " " => let letter = "u";
		        " " => let letter = "u";
	            " " => let letter = "v";
		        " " => let letter = "v";
	            " " => let letter = "w";
		        " " => let letter = "w";
	            " " => let letter = "x";
		        " " => let letter = "x";
	            " " => let letter = "y";
		        " " => let letter = "y";
	            " " => let letter = "z";
		        " " => let letter = "z";
	            " " => let letter = "0";
	            " " => let letter = "1";
	            " " => let letter = "2";
	            " " => let letter = "3";
	            " " => let letter = "4";
	            " " => let letter = "5";
	            " " => let letter = "6";
	            " " => let letter = "7";
	            " " => let letter = "8";
	            " " => let letter = "9";
                " " => let letter = " ";
		        " " => let letter = "!";
	    }
	    
        if caps = 1 {
            match scancode {
                " " => let letter = "A"; // make code
		        " " => let letter = "A";
	            " " => let letter = "B";
		        " " => let letter = "B"; 
	            " " => let letter = "C";
		        " " => let letter = "C";
	            " " => let letter = "D";
		        " " => let letter = "D";
	            " " => let letter = "E";
		        " " => let letter = "E";
	            " " => let letter = "F";
		        " " => let letter = "F";
	            " " => let letter = "G";
		        " " => let letter = "G";
	            " " => let letter = "H";
		        " " => let letter = "H";
	            " " => let letter = "I";
		        " " => let letter = "I";
	            " " => let letter = "J";
		        " " => let letter = "J";
	            " " => let letter = "K";
		        " " => let letter = "k";
	            " " => let letter = "l";
		        " " => let letter = "l";
	            " " => let letter = "m";
		        " " => let letter = "m";
	            " " => let letter = "n";
		        " " => let letter = "n";
	            " " => let letter = "o";
		        " " => let letter = "o";
	            " " => let letter = "p";
		        " " => let letter = "p";
	            " " => let letter = "q";
		        " " => let letter = "q";
	            " " => let letter = "r";
		        " " => let letter = "r";
	            " " => let letter = "s";
		        " " => let letter = "s";
	            " " => let letter = "t";
		        " " => let letter = "t";
	            " " => let letter = "u";
		        " " => let letter = "u";
	            " " => let letter = "v";
		        " " => let letter = "v";
	            " " => let letter = "w";
		        " " => let letter = "w";
	            " " => let letter = "x";
		        " " => let letter = "x";
	            " " => let letter = "y";
		        " " => let letter = "y";
	            " " => let letter = "z";
		        " " => let letter = "z";
	            " " => let letter = "0";
	            " " => let letter = "1";
	            " " => let letter = "2";
	            " " => let letter = "3";
	            " " => let letter = "4";
	            " " => let letter = "5";
	            " " => let letter = "6";
	            " " => let letter = "7";
	            " " => let letter = "8";
	            " " => let letter = "9";
                " " => let letter = " ";
		        " " => let letter = "!";
            }
	
	
        command.push_str(letter)
	    
        match command {
	    "scrypt" => let command = 1;
	    "comms" => let command = 2;
	    }  
        
        // bash commands
        if command = 1010 { // shell script_default

        }

        if command = 10100 { // new shell script

        }
    
        if command = 1100 { // run

        }

        if command = 1111 { // info
            println!("Project Longhorn v0.1");
		    println!("A Community-Made OS");
		    println!("------------------------");
		    println!("Developers: ");
		    println!("Hayden Curfman, project leader/ lead programmer");
        }

        // non bash stuff
        if command = 1 { // scrypt
		    scrypt::editor();
        }
    
        if command = 2 { // comms 

        }
    }
}

#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop{}
}
