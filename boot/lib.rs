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
mod comms;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
    use memory::FrameAllocator;

    vga_buffer::clear_screen();
    let version = String::from("v0.0.1");// needs to read version from a file

    let mut memory = 1;
    
    if memory = 1 {
        println!("Memory/Paging Tables:")
        let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
        let memory_map_tag = boot_info.memory_map_tag()
            .expect("Memory map tag required");

        println!("memory areas:");
        for area in memory_map_tag.memory_areas() {
            println!("    start: 0x{:x}, length: 0x{:x}",
                area.base_addr, area.length);
        }

        let elf_sections_tag = boot_info.elf_sections_tag()
            .expect("Elf-sections tag required");

        println!("kernel sections:");
        for section in elf_sections_tag.sections() {
            println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
                section.addr, section.size, section.flags);
        }

        let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
            .min().unwrap();
        let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
            .max().unwrap();
        let multiboot_start = multiboot_information_address;
        let multiboot_end = multiboot_start + (boot_info.total_size as usize);

        let mut frame_allocator = memory::AreaFrameAllocator::new(
            kernel_start as usize, kernel_end as usize, multiboot_start,
            multiboot_end, memory_map_tag.memory_areas());
    
        println!("");
        
    }
    // terminal time
    println!("Project Longhorn, {}", version);
    let mut keyboard: Port<u8> = unsafe { Port::new(0x60) };
    let command = String::new();
    let scancode = keyboard.read();
    if caps = 0 {
        match scancode {
	        " " => let letter = "a";
	        " " => let letter = "b";
	        " " => let letter = "c";
	        " " => let letter = "d";
	        " " => let letter = "e";
	        " " => let letter = "f";
	        " " => let letter = "g";
	        " " => let letter = "h";
	        " " => let letter = "i";
	        " " => let letter = "j";
	        " " => let letter = "k";
	        " " => let letter = "l";
	        " " => let letter = "m";
	        " " => let letter = "n";
	        " " => let letter = "o";
	        " " => let letter = "p";
	        " " => let letter = "q";
	        " " => let letter = "r";
	        " " => let letter = "s";
	        " " => let letter = "t";
	        " " => let letter = "u";
	        " " => let letter = "v";
	        " " => let letter = "w";
	        " " => let letter = "x";
	        " " => let letter = "y";
	        " " => let letter = "z";
	        // numbers use the letter variable because it is faster
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
	        // special keys also use the letter variable, see above
            	" " => let letter = " ";
		" " => let letter = "!";

        }
	    
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
