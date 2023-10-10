#[derive(Debug)]
struct MyMap<K: Ord, V: Clone> {
    keys: Vec<K>,
    values: Vec<V>,
    length: usize,
}

impl <K: Ord, V: Clone> MyMap<K, V> {
    fn new() -> MyMap<K, V> {
        MyMap {
            keys: Vec::new(),
            values: Vec::new(),
            length: 0,
        }
    }

    fn has_key(&self, key: &K) -> bool {
        for i in 0..self.length {
            if self.keys[i] == *key {
                return true;
            }
        }
        
        false
    }

    fn insert(&mut self, key: K, value: V) {
        if self.has_key(&key) {
            for i in 0..self.length {
                if self.keys[i] == key {
                    self.values[i] = value.clone();
                }
            }
        } else {
            self.keys.push(key);
            self.values.push(value);
            self.length += 1;
        }

    }
}


enum Colour {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Colour {
    fn RGB(&self) -> &str {
        match self {
            Colour::Red => {
                "FF0000"
            },
            Colour::Green => {
                "00FF00"
            },
            Colour::Blue => {
                "0000FF"
            },
            Colour::Yellow => {
                "00FFFF"
            }
        }
    }
}

enum Type {
    number (f64),
    point (i32, i32),
    substring (String, usize),
}

struct op {
    pub fp: fn(String) -> bool,
}

impl op {
    fn new(f: fn(String) -> bool) -> op {
        op {
            fp: f
        }
    }
}

fn starts_with_A(s: String) -> bool {
    s.starts_with("A")
}


fn main() {
    let a = op::new(starts_with_A);

    (a.fp)("Hello".to_string());
}
