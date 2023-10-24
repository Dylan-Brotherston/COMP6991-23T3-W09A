fn heyyo() {
    let x = examplefn::<i32>;
    let y = examplefn::<u32>;
    givemepointer(x);
    givemepointer(y);
    givemepointerFn(x);
    givemepointerFn(y);
}

fn examplefn<T>(a: i32) -> i32 {
    0
}

fn givemepointer(f: fn(i32) -> i32) {}

fn givemepointerFn<T>(f: T)
where
    T: Fn(i32) -> i32,
{
}
