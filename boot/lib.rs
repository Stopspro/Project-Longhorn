#![feature(lang_items)]
#![feature(const_fn)]
#![feature(const_unique_new)]
#![feature(unique)]
#![no_std]

#[macro_use]
extern crate bitflags;
extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;
extern crate x86_64;
extern crate cpuio;

use cpuio::Port;
use cpuio::UnsafePort;

#[macro_use]
mod vga_buffer;
mod memory;
mod keyboard;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
    use memory::FrameAllocator;
	
    // set up guard page and map the heap pages
    let mut memory_controller = memory::init(boot_info); // new return type

    // initialize our IDT
    interrupts::init(&mut memory_controller); // new argument	

    vga_buffer::clear_screen();
	
    // Interrupt Setup
    pub struct UnsafePort<T: InOut> {
    	port: u16,
    	phantom: PhantomData<T>,
	}

	impl<T: InOut> UnsafePort<T> {
    	pub const unsafe fn new(port: u16) -> UnsafePort<T> {
        	UnsafePort { port: port, phantom: PhantomData }
    	}

    	pub unsafe fn read(&mut self) -> T {
        	T::port_in(self.port)
    	}

    	pub unsafe fn write(&mut self, value: T) {
        	T::port_out(self.port, value);
    	}
	}	

    pub trait InOut {
        unsafe fn port_in(port: u16) -> Self;
        unsafe fn port_out(port: u16, value: Self);
    }

    impl InOut for u8 {
        unsafe fn port_in(port: u16) -> u8 { inb(port) }
        unsafe fn port_out(port: u16, value: u8) { outb(value, port); }
    }

    impl InOut for u16 {
        unsafe fn port_in(port: u16) -> u16 { inw(port) }
        unsafe fn port_out(port: u16, value: u16) { outw(value, port); }
    }

    impl InOut for u32 {
        unsafe fn port_in(port: u16) -> u32 { inl(port) }
        unsafe fn port_out(port: u16, value: u32) { outl(value, port); }
    }
        
    }
    // terminal time
    println!("Project Longhorn, {}", version);
	print!("Username: ")
    let command = String::new();
    loop {
        if caps = 0 {
            match scancode {
    	        "1C" => let letter = "a-BREAK"; // make code
	    	    "F01C" => let letter = "a"; // break code 
	            "32" => let letter = "b"; // repeat
    		    "F032" => let letter = "b-BREAK"; 
	            "21" => let letter = "c";
    		    "F021" => let letter = "c-BREAK";
	            "23" => let letter = "d";
    		    "F023" => let letter = "d-BREAK";
	            "24" => let letter = "e";
    		    "F024" => let letter = "e-BREAK";
	            "2B" => let letter = "f";
	    	    "F02B" => let letter = "f-BREAK";
	            "34" => let letter = "g";
	    	    "F034" => let letter = "g-BREAK";
	            "33" => let letter = "h";
	    	    "F034" => let letter = "h-BREAK";
	            "43" => let letter = "i";
	    	    "F043" => let letter = "i-BREAK";
	            "3B" => let letter = "j";
	            "F03B" => let letter = "j-BREAK";
	            "42" => let letter = "k";
		        "F042" => let letter = "k-BREAK";
	            "4B" => let letter = "l";
		        "F04B" => let letter = "l-BREAK";
	            "3A" => let letter = "m";
		        "F03A" => let letter = "m-BREAK";
	            " " => let letter = "n";
		        " " => let letter = "n-BREAK";
	            " " => let letter = "o";
		        " " => let letter = "o-BREAK";
	            " " => let letter = "p";
		        " " => let letter = "p-BREAK";
	            " " => let letter = "q";
		        " " => let letter = "q-BREAK";
	            " " => let letter = "r";
		        " " => let letter = "r-BREAK";
	            " " => let letter = "s";
		        " " => let letter = "s-BREAK";
	            " " => let letter = "t";
		        " " => let letter = "t-BREAK";
	            " " => let letter = "u";
		        " " => let letter = "u-BREAK";
	            " " => let letter = "v";
		        " " => let letter = "v-BREAK";
	            " " => let letter = "w";
		        " " => let letter = "w-BREAK";
	            " " => let letter = "x";
		        " " => let letter = "x-BREAK";
	            " " => let letter = "y";
		        " " => let letter = "y-BREAK";
	            " " => let letter = "z";
		        " " => let letter = "z-BREAK";
	            " " => let letter = "0";
                " " => let letter = "0-BREAK";
	            " " => let letter = "1";
                " " => let letter = "1-BREAK";
                " " => let letter = "2";
	            " " => let letter = "2-BREAK";
	            " " => let letter = "3";
                " " => let letter = "3-BREAK";
	            " " => let letter = "4";
                " " => let letter = "4-BREAK";
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
		        " " => let letter = "A-BREAK"; // break code
	            " " => let letter = "B"; // repeat
		        " " => let letter = "B-BREAK"; 
	            " " => let letter = "C";
		        " " => let letter = "C-BREAK";
	            " " => let letter = "D";
		        " " => let letter = "D-BREAK";
	            " " => let letter = "E";
		        " " => let letter = "E-BREAK";
	            " " => let letter = "F";
		        " " => let letter = "F-BREAK";
	            " " => let letter = "G";
		        " " => let letter = "G-BREAK";
	            " " => let letter = "H";
		        " " => let letter = "H-BREAK";
	            " " => let letter = "I";
		        " " => let letter = "I-BREAK";
	            " " => let letter = "J";
		        " " => let letter = "J-BREAK";
	            " " => let letter = "K";
		        " " => let letter = "K-BREAK";
	            " " => let letter = "L";
		        " " => let letter = "L-BREAK";
	            " " => let letter = "M";
		        " " => let letter = "M-BREAK";
	            " " => let letter = "N";
		        " " => let letter = "N-BREAK";
	            " " => let letter = "O";
		        " " => let letter = "O-BREAK";
	            " " => let letter = "P";
		        " " => let letter = "P-BREAK";
	            " " => let letter = "Q";
		        " " => let letter = "Q-BREAK";
	            " " => let letter = "R";
		        " " => let letter = "R-BREAK";
	            " " => let letter = "S";
		        " " => let letter = "S-BREAK";
	            " " => let letter = "T";
		        " " => let letter = "T-BREAK";
	            " " => let letter = "U";
		        " " => let letter = "U-BREAK";
	            " " => let letter = "V";
		        " " => let letter = "V-BREAK";
	            " " => let letter = "W";
		        " " => let letter = "W-BREAK";
	            " " => let letter = "X";
		        " " => let letter = "X-BREAK";
	            " " => let letter = "Y";
		        " " => let letter = "Y-BREAK";
	            " " => let letter = "Z";
		        " " => let letter = "Z-BREAK";
	            " " => let letter = "0";
	            " " => let letter = "1-BREAK";
	            " " => let letter = "2";
	            " " => let letter = "3-BREAK";
	            " " => let letter = "4";
	            " " => let letter = "5-BREAK";
	            " " => let letter = "6";
				" " => let letter = "6-BREAK";
	            " " => let letter = "7";
	            " " => let letter = "8";
	            " " => let letter = "9";
                " " => let letter = " ";
		        " " => let letter = "!";
            }
		}
		
        command.push_str(letter)
	    
        match command {
	    "dir" => let command = 1;
		"mkdir" = let command = 2;
	    "scrypt" => let command = 4;
		"about" => let command = 7;
		"settings" => let command = 8;
	    }  
        
		if command = 1 { // dir
			print("{}", dir);
		}
		
		if command = 2 { // make dir alias mkdir
				
		}
			
		if command = 3{ // clear
			vga_buffer::clear_screen();
		}
			
        if command = 4 { // scrypt
			scrypt::editor();
        }
			
		if command = 5 { // rmdir
			
		}
			
		if command = 6 { // logout
			
		}

        if command = 7 { // about
            println!("Project Longhorn v0.1");
		    println!("A Community-Made OS");
		    println!("------------------------");
		    println!("Developers: ");
		    println!("Hayden Curfman, project leader/ lead programmer");
        }

        if command = 8 { // settings
		    
        }
			
		if command = 9 { // ip
				
		}
			
		if command = 10 { // networking setup
			// find type, wifi or ethernet
			if type = 1 { // ethernet
			
			}
			
			if type = 2 {
				
			}
		}
			
		if command = 0 {
			println!("Error; command not recognized.")	
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
