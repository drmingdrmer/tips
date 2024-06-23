// https://github.com/SkyFan2002/databend/commit/2e4d28a7714b5f71294eefd463c4840ab7b9d038

// thread_local! {
//     static RSP: isize = get_rsp();
// }

// pub fn get_rsp() -> isize {
//     let rsp: isize;
//     unsafe {
//         core::arch::asm!("mov {}, rsp", out(reg) rsp);
//     }
//     rsp
// }
// pub fn check() {
//     let rsp = get_rsp();
//
//     let base = RSP.with(|&v| v);
//
//     let stack_size = base - rsp;
//
//     let m = stack_size / 1024 / 1024;
//
//     let k = (stack_size - m * 1024 * 1024) / 1024;
//
//     let b = stack_size - m * 1024 * 1024 - k * 1024;
//
//     println!("stack size: {}M {}K {}B", m, k, b);
// }
