// this implementation of unicorn is a Rust program that can (and will) test and run Assembly code.
extern crate unicorn

use unicorn::{Cpu, CpuX86};
pub fn cpu() {
    let x86_code32: Vec<u8> = vec![0x41, 0x4a]; // INC ecx; DEC edx

    let emu = CpuX86::new(unicorn::Mode::MODE_32).expect("failed to instantiate emulator");
    let _ = emu.mem_map(0x1000, 0x4000, unicorn::PROT_ALL);
    let _ = emu.mem_write(0x1000, &x86_code32);
    let _ = emu.reg_write_i32(unicorn::RegisterX86::ECX, -10);
    let _ = emu.reg_write_i32(unicorn::RegisterX86::EDX, -50);

    let _ = emu.emu_start(0x1000, (0x1000 + x86_code32.len()) as u64, 10 * unicorn::SECOND_SCALE, 1000);
    assert_eq!(emu.reg_read_i32(unicorn::RegisterX86::ECX), Ok((-9)));
    assert_eq!(emu.reg_read_i32(unicorn::RegisterX86::EDX), Ok((-51)));
}
