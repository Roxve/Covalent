#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn writefn_ptr__i8(s: *const i8) {
    let c_str = unsafe {
        assert!(!s.is_null());
        std::ffi::CStr::from_ptr(s as *const u8)
    };
    println!("{}", c_str.to_str().unwrap());
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn writefn_i32(i: i32) {
    println!("{}", i);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn writefn_float(f: f32) {
    println!("{}", f);
}
