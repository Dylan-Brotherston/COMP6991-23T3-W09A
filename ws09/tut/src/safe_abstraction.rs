struct SafeWrapper {
    ptr: *mut i32,
}

impl SafeWrapper {
    fn new() -> Self {
        let b = Box::new(42);
        SafeWrapper {
            ptr: Box::into_raw(b),
        }
    }

    fn get(&self) -> i32 {
        unsafe { *self.ptr }
    }

    fn set(&mut self, value: i32) {
        unsafe {
            *self.ptr = value;
        }
    }
}

impl Drop for SafeWrapper {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.ptr);
        }
    }
}

fn main() {
    let mut wrapper = SafeWrapper::new();
    println!("Value: {}", wrapper.get());
    wrapper.set(55);
    println!("Value: {}", wrapper.get());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        main();
    }
}
