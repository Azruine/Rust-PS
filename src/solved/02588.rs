fn main()
{
    let mut a = String::new();
    let mut b = String::new();
    std::io::stdin().read_line(&mut a).unwrap();
    std::io::stdin().read_line(&mut b).unwrap();
    let a: i32 = a.trim().parse().unwrap();
    let b: i32 = b.trim().parse().unwrap();
    println!("{}", a * (b % 10));
    println!("{}", a * ((b % 100) / 10));
    println!("{}", a * (b / 100));
    println!("{}", a * b);
}