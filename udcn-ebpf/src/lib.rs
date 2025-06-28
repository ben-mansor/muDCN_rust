#![no_std]

// Minimal eBPF program placeholder
#[no_mangle]
pub extern "C" fn xdp_prog() -> u32 {
    0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

