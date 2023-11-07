unsafe fn dangerous_function() {}

fn main() {
    let result;
    unsafe {
        result = dangerous_function();
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
