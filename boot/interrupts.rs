use x86_64::structures::idt::ExceptionStackFrame;

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init() {
    IDT.load();
}
extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut ExceptionStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

// Sample Usage
// in src/lib.rs

// pub extern "C" fn rust_main(...) {
    // ...
    // memory::init(boot_info);

    // initialize our IDT
    // interrupts::init();

    // invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3();

    // println!("It did not crash!");
    // loop {}
// }
