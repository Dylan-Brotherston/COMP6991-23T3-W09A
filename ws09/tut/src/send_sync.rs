use std::sync::{Arc, Mutex};
use std::thread;

struct MySyncStruct {
    data: Mutex<i32>, // Mutex allows us to safely mutate data even when shared between threads
}

// In Rust, the Sync trait is automatically implemented for types that the compiler can guarantee are safe to share across threads. This typically includes types with immutable shared state or types that internally manage synchronization and mutable state (like Mutex<T>).

// However, this automatic implementation does not extend to types containing raw pointers (`*mut T` or `*const T`) because raw pointers can violate thread-safety. Raw pointers allow for mutable aliasing without any concurrency control, making them inherently unsafe to share between threads.

// Therefore, when a struct contains raw pointers, the compiler cannot automatically implement the Sync trait, as it cannot ensure that the data being pointed to is accessed in a thread-safe manner.

// By writing `unsafe impl Sync for MySyncStruct`, we are asserting that despite the presence of raw pointers, we have manually ensured thread safety. This might involve using atomic operations, explicit locking mechanisms, or other forms of synchronization to manage access to the raw pointer.

// This implementation is marked as `unsafe` because it's a promise to the compiler and other programmers that you are manually upholding Rust's thread-safety guarantees. It indicates that you have ensured exclusive access to the mutable data, preventing data races and other undefined behavior that can arise from concurrent access.

// The `unsafe` keyword here is a sign that the programmer must be very cautious; it's their responsibility to uphold the safety invariants that Sync requires. This should only be done when you are absolutely certain that your manual synchronization strategy is sound.

// unsafe impl Sync for MySyncStruct {}

fn main() {
    let my_struct = Arc::new(MySyncStruct {
        data: Mutex::new(0),
    });

    let mut handles = vec![];

    for _ in 0..10 {
        let my_struct_clone = Arc::clone(&my_struct);
        let handle = thread::spawn(move || {
            let mut data = my_struct_clone.data.lock().unwrap();
            *data += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *my_struct.data.lock().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        main();
    }
}
