fn main() {
    let mut n: String = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();
    for _ in 0..n {
        let mut t = 0;
        let mut a = Default::default();
        let mut b = Default::default();
        std::io::stdin().read_line(&mut a).unwrap();
        std::io::stdin().read_line(&mut b).unwrap();
        a.chars().zip(b.chars()).for_each(|(x, y)| {
            if x != y {
                t += 1;
            }
        });
        println!("Hamming distance is {}.", t);
    }
}
