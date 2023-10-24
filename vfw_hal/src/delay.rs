#[linkage = "weak"]
#[no_mangle]
pub extern "C" fn delay_us(_us: u32) {
    panic!("delay_us not supported!")
}

#[linkage = "weak"]
#[no_mangle]
pub extern "C" fn delay_ms(_ms: u32) {
    panic!("delay_ms not supported!")
}
