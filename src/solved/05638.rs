#[allow(non_snake_case)]

fn main()
{
    let INT32_MAX = 2147483647;
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();
    let mut flow = vec![0; n as usize];
    let mut cost = vec![0; n as usize];
    for i in 0..n
    {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let s: Vec<i32> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
        flow[i as usize] = s[0];
        cost[i as usize] = s[1];
    }
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t: i32 = t.trim().parse().unwrap();
    for i in 1..=t
    {
        let mut str = String::new();
        std::io::stdin().read_line(&mut str).unwrap();
        let str: Vec<i32> = str.split_whitespace().map(|x| x.parse().unwrap()).collect();
        // str[0] = total volume, str[1] = max time
        let mut minCost = INT32_MAX;

        'outLoop: for j in 1..=(1<<n)
        {
            let mut totalFlow = 0;
            let mut totalCost = 0;
            for k in 0..n
            {
                if (j & (1<<k)) != 0
                {
                    totalFlow += flow[k as usize];
                    totalCost += cost[k as usize];
                    if totalCost >= minCost
                    {
                        continue 'outLoop;
                    }
                }
            }
            if str[0] <= str[1] * totalFlow
            {
                minCost = std::cmp::min(minCost, totalCost);
            }
        }
        if minCost == INT32_MAX
        {
            println!("Case {}: IMPOSSIBLE", i);
        }
        else
        {
            println!("Case {}: {}", i, minCost);
        }
    }
}