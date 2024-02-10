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

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn strcat_ptr__i8(s: *const i8, s2: *const i8) -> *const i8 {
    let c_str = unsafe {
        assert!(!s.is_null());
        std::ffi::CStr::from_ptr(s as *const u8)
    };

    let c_str2 = unsafe {
        assert!(!s.is_null());
        std::ffi::CStr::from_ptr(s2 as *const u8)
    };
    let res = c_str.to_str().unwrap().to_owned() + c_str2.to_str().unwrap() + "\0";
    let res_box = Box::new(res);
    // let res_ptr = res_box.as_bytes().as_ptr();
    return unsafe { Box::into_raw(res_box).as_ref().unwrap().as_ptr() as *const i8 };
}
