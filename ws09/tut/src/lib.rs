mod ffi;
mod locking_primitives;
mod raw_pointers;
mod rc_eg;
mod safe_abstraction;
mod send_sync;
mod threading_eg;
mod unsafe_eg;
mod ws09;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
