use std::rc::Rc;

fn main() {
    let five = Rc::new(5);

    let five_clone1 = Rc::clone(&five);
    let five_clone2 = Rc::clone(&five);

    println!("Count after cloning twice: {}", Rc::strong_count(&five));

    drop(five_clone1);
    println!("Count after dropping one: {}", Rc::strong_count(&five));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        main();
    }
}
