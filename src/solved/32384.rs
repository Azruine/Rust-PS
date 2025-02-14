fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse().unwrap();
    for i in 0..n {
        print!("LoveisKoreaUniversity");
        if i < n {
            print!(" ");
        }
    }
}
