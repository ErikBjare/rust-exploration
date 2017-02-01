extern crate libc;
extern crate nalgebra;

#[no_mangle]
pub mod astro;

#[no_mangle]
pub mod threadcount;

#[repr(C)]
pub struct Something {
    i: i32,
    j: i32,
}

#[no_mangle]
pub extern fn do_something() {
    println!("asd")
}

#[no_mangle]
pub extern fn return_int(i: i32) -> i32 {
    return i + 1;
}

#[no_mangle]
pub extern fn return_something(i: i32, j: i32) -> Something {
    let s = Something { i: i, j: j };
    println!("asd");
    return s;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
