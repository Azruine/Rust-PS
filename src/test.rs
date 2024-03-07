fn main()
{
    let mut a: String = String::new();
    std::io::stdin().read_line(&mut a).expect("Read failed");
    println!("{}", a)
}