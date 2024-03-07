fn main()
{
    let mut a: String = String::new();
    let mut flag: bool = true;
    std::io::stdin().read_line(&mut a).unwrap();
    for _ in 0..3
    {
        let mut b: String = String::new();
        std::io::stdin().read_line(&mut b).unwrap();
        flag = !b.split_whitespace().map(|s: &str| s.parse().unwrap()).collect().contains(&7);
    }
    if flag
    {
        println!("777");
    }
    else
    {
        println!("0");
    }
}