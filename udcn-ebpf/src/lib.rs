#![no_std]

// Minimal XDP program that always passes packets.
#[no_mangle]
pub extern "C" fn xdp_pass(_ctx: *mut core::ffi::c_void) -> u32 {
    const XDP_PASS: u32 = 2;
    XDP_PASS
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
