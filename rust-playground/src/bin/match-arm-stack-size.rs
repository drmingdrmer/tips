use std::fmt::Display;

static mut RSP: isize = 0;

fn print_stack_pos(msg: impl Display) {
    let a = ();
    let ptr = &a as *const _ as isize;
    let prev = unsafe { RSP };

    println!("{:>5} stack ptr: {:x}, diff: {}", msg, ptr, prev - ptr);

    unsafe {
        RSP = ptr;
    }
}

fn inner() -> u8 {
    print_stack_pos("inner");
    0
}

fn outer() {
    let v = 0;
    // Every matching arm occupies a piece of memory on the stack.
    // Thus the stack memory for `inner()` starts will be 3KB after on the stack.
    match v {
        _ if v == 0 => inner(),
        _ if v == 1 => [0u8; 1024][0],
        _ if v == 2 => [0u8; 1024 * 2][0],
        _ => 0,
    };
}

fn main() {
    print_stack_pos("main");
    outer();
}
