use std::thread;

fn main() {
    let s = String::new();

    let dropstr = || println!("{}", s);

    print!("{}", s);
}

fn realistic_example() {
    let v = vec![1, 2, 3];
    let fnPtr = i32::to_string;
    v.iter().map(i32::to_string);
}
fn realistic_examplev2() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
