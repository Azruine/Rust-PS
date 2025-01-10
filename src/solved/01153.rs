fn sieve() -> Vec<bool>
{
    let mut prime = vec![true; 1000001];
    prime[0] = false;
    prime[1] = false;
    for i in 2..1001
    {
        if prime[i]
        {
            for j in (i * i..1000001).step_by(i)
            {
                prime[j] = false;
            }
        }
    }
    prime
}

fn solve(n: i32, prime: &Vec<bool>)
{
    let mut temp = n;
    if n < 8
    {
        println!("-1");
        return;
    }
    else if n & 1 == 1
    {
        print!("2 3");
        temp -= 5;
    }
    else
    {
        print!("2 2");
        temp -= 4;
    }
    for i in 2..temp
    {
        if prime[i as usize] && prime[(temp - i) as usize]
        {
            println!(" {} {}", i, temp - i);
            return;
        }
    }
}

fn main()
{
    let prime = sieve();
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    solve(n.trim().parse().unwrap(), &prime);
}
