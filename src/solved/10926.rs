fn main() {
    let mut a: String = String::new();
    std::io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");
    print!("{}??!", a.trim_end());
}
