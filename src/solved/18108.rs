fn main()
{
    let mut a: String = String::new();
    std::io::stdin().read_line(&mut a).expect("Failed to read line");
    let b: i64 = a.trim_end().parse().expect("Failed to parse");
    println!("{}", b - 543);
}