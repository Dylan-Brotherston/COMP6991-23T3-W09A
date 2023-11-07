use libc::{c_char, c_double, c_int, fclose, fgets, fopen, fscanf, FILE};

struct File {
    file: *mut FILE,
}

/// This function converts a string into a Vec<i8> which can
/// be used to represent a c-string.
///
/// To turn this into a string, use Vec<i8>::as_mut_ptr().
fn to_c_string(string: &str) -> Vec<i8> {
    let bytes: Vec<u8> = String::from(string).into_bytes();
    let mut c_chars: Vec<i8> = bytes.iter().map(|c| *c as i8).collect();

    c_chars.push(0); // null terminator

    c_chars
}

impl File {
    fn open(path: &str) -> Option<Self> {
        unsafe {
            let mut name = to_c_string(path);
            let mut mode = to_c_string("r");
            let f = fopen(name.as_mut_ptr(), mode.as_mut_ptr());
            if f.is_null() {
                None
            } else {
                Some(File { file: f })
            }
        }
    }

    fn read_string(&mut self) -> Option<String> {
        let mut my_string = Vec::new();
        my_string.resize(1000, 0);
        let ok = unsafe { fgets(my_string.as_mut_ptr(), 1000, self.file) };
        if ok.is_null() {
            None
        } else {
            String::from_utf8(my_string.iter().map(|i| *i as u8).collect()).ok()
        }
    }

    fn read_i64(&mut self) -> Option<i64> {
        unsafe {
            let mut fmtd = to_c_string("%d");
            let i: c_int = 0;
            let amt_scanned = fscanf(self.file, fmtd.as_mut_ptr(), &i);
            if amt_scanned != 1 {
                None
            } else {
                Some(i.into())
            }
        }
    }

    fn read_f64(&mut self) -> Option<f64> {
        unsafe {
            let mut fmtd = to_c_string("%lf");
            let i: c_double = 0.0;
            let amt_scanned = fscanf(self.file, fmtd.as_mut_ptr(), &i);
            if amt_scanned != 1 {
                None
            } else {
                Some(i)
            }
        }
    }

    fn read_char(&mut self) -> Option<char> {
        unsafe {
            let mut fmtd = to_c_string(" %c");
            let i: c_char = 0;
            let amt_scanned = fscanf(self.file, fmtd.as_mut_ptr(), &i);
            if amt_scanned != 1 {
                None
            } else {
                char::from_u32(i as u32)
            }
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            if !self.file.is_null() {
                fclose(self.file);
            }
        }
    }
}

fn main() {
    let mut file = File::open("./src/data/test_file.txt").expect("This should work.");
    let s = file.read_string().unwrap();
    let i = file.read_i64().unwrap();
    let f = file.read_f64().unwrap();
    let c = file.read_char().unwrap();

    println!("{s} {i} {f} {c}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        main();
    }
}
