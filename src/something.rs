#[repr(C)]
pub struct Something {
    i: i32,
    j: i32,
}

#[no_mangle]
pub extern fn print_something() {
    println!("I'm printing from Rust!")
}

#[no_mangle]
pub extern fn return_int_plus_one(i: i32) -> i32 {
    return i + 1;
}

#[no_mangle]
pub extern fn return_something(i: i32, j: i32) -> Something {
    return Something { i: i, j: j };
}
