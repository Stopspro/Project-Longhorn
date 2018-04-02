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
extern crate ion;
extern crate neutron;
extern crate electron;

#[macro_use]
mod vga_buffer;
mod memory;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
    use memory::FrameAllocator;

    vga_buffer::clear_screen();
    let version = String::from("v0.0.1");// needs to read version from a file

    let command = 

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
    
    // terminal time
    println!("atomOS, { }", version);
    particle::main::shell();
    
    if ion == 1 {                   
        println!("Loading bypassed, directly installing ion shell...")
        ion::load::shell();
        println!("Done. installing shell commands...")
        ion::load::commands();
        println!("Done. installing shell themes...")
        ion::load::themes();
        println!("ion installed.")
        ion::shell::reboot();
    }
    
    if neutron == 1 {         
        println!("Loading bypassed, directly installing neutron shell...")
        neutron::load::shell()
        println!("Done. installing shell commands...")
        neutron::load::commands();
        println!("Done. installing shell themes...")
        neutron::load::themes();
        println!("ion installed.")
        neutron::shell::reboot();
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

// current ouput:
// Memory/Paging:
// a bunch of memory/paging stuff
// atomOS, v0.1

// desired output:
// Memory/Paging:
// a bunch of memory/paging stuff
// Readying packages... Done
// (big fancy atomOS sign made from /)
// v0.1
// >>>
