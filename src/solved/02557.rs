use std::io::Write;

fn main() {
    let s: String = String::from("Hello World!\n");
    std::io::stdout().write_all(s.as_bytes()).unwrap();
}
