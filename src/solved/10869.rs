fn main()
{
    let mut a: String = String::new();
    std::io::stdin().read_line(&mut a).expect("Failed to read line");
    let numbers: Vec<i64> = a.split_whitespace().map(|s: &str| s.parse().unwrap()).collect();
    println!("{}", numbers[0] + numbers[1]);
    println!("{}", numbers[0] - numbers[1]);
    println!("{}", numbers[0] * numbers[1]);
    println!("{}", numbers[0] / numbers[1]);
    println!("{}", numbers[0] % numbers[1]);
}