fn main()
{
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let mut iter = string.split_whitespace();
    let n: i64 = iter.next().unwrap().parse::<i64>().unwrap();
    let k: i64 = iter.next().unwrap().parse::<i64>().unwrap();
    let m: i64 = iter.next().unwrap().parse::<i64>().unwrap();
    let mut a: Vec<i64> = Vec::new();
    let mut ans = -1;
    let mut r = 0;
    for _ in 0..n
    {
        let mut string = String::new();
        std::io::stdin().read_line(&mut string).unwrap();
        let temp: i64 = string.trim().parse().unwrap();
        if temp >= 2 * k
        {
            a.push(temp - 2 * k);
            r = std::cmp::max(r, temp - 2 * k);
        }
        else if temp >= k
        {
            a.push(temp - k);
            r = std::cmp::max(r, temp - k);
        }
        else
        {   
            continue;
        }   
    }
    if a.len() as i64 == 0
    {
        println!("{}", ans);
    }
    else
    {
        let mut l = 1;
        while l <= r
        {
            let mid = (l + r) / 2;
            let mut sum = 0;
            for i in a.iter()
            {
                sum += i / mid;
            }
            if sum >= m
            {
                ans = mid;
                l = mid + 1;
            }
            else
            {
                r = mid - 1;
            }
        }
        println!("{}", ans);
    }
}