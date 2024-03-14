fn main()
{
    let mut a: String = String::new();
    std::io::stdin().read_line(&mut a).expect("Read failed");
    let a: Vec<i32> = a.split_whitespace().map(|x: &str| x.parse().unwrap()).collect();
    let (a, b, c) = (a[0], a[1], a[2]);
    println!("{}", (a+b)%c);
    println!("{}", ((a%c) + (b%c))%c);
    println!("{}", (a*b)%c);
    println!("{}", ((a%c) * (b%c))%c);
}