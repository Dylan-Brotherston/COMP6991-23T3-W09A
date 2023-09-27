fn print_length(s: &String) {
    println!("{} is {} characters long", s, s.len());
}

fn print_first_and_last(s: &String) {
    let first = s.chars().next().unwrap();
    let last = s.chars().last().unwrap();
    println!("{} begins with {} and ends with {}", s, first, last);
}

fn add_2(list: &mut Vec<i32>) {
    let _ = list.iter_mut().map(|f| *f + 2);
}

#[derive(Copy, Clone)]
struct data {
    x: i32,
    y: i32,
}

fn main() {
    let prompt = "Hello, World!".to_string();
    println!("prompt: {}", prompt);
    print_length(&prompt);
    print_first_and_last(&prompt);
    println!("prompt: {}", prompt);

    let mut list = vec![1,2,3,4,5];
    println!("{:?}", list);
    add_2(&mut list);
    println!("{:?}", list);
}
