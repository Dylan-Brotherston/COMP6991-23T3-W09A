fn hey() {
    let x = examplefn::<i32>;
    // x = examplefn::<u32>; // wont compile
}

fn examplefn<T>() {}
