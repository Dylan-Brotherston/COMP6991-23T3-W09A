use std::thread;

mod closures;
mod fn_items;
mod fun_ptrs;
mod my_macs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = my_vec!(1, 2, 3);
        println!("{:?}", a);
        say_hello!();
    }
}
