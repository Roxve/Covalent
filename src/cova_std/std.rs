#[repr(C)]
pub struct Obj {
    bytes: [i8; 4],
    ty: i8,
    string: *const i8,
}

#[no_mangle]
pub extern "C" fn test(ob: *const Obj) -> *const Obj {
    unsafe {
        println!("{}", (*ob).ty);
        ob
    }
}
