fn main() {
    let x = 10;
    let r = &x as *const i32; // Immutable raw pointer to x

    let mut y = 20;
    let m = &mut y as *mut i32; // Mutable raw pointer to y

    let address = 0x0123435usize;
    let raw_addr = address as *mut usize;

    unsafe {
        println!("r points to: {}", *r);
        *m = 30; // Changing the value of y through a raw pointer also requires an unsafe block
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        main();
    }
}
