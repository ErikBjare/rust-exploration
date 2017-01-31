#[no_mangle]
pub mod threadcount;

#[no_mangle]
pub extern fn do_something() {
    println!("asd")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
