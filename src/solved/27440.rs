use std::collections::hash_set::HashSet;
use std::collections::VecDeque;

#[allow(non_snake_case)]

fn main() {
    let mut str = String::new();
    std::io::stdin().read_line(&mut str).unwrap();
    let n: u64 = str.trim().parse().unwrap();
    let mut q: VecDeque<(u64, u64)> = VecDeque::new();
    q.push_back((n, 0));
    let mut visited: HashSet<u64> = HashSet::new();
    while !q.is_empty() {
        let (x, d) = q.pop_front().unwrap();
        if x == 1 {
            println!("{}", d);
            return;
        }
        if visited.contains(&x) {
            continue;
        }
        visited.insert(x);
        if x % 3 == 0 {
            q.push_back((x / 3, d + 1));
        }
        if x % 2 == 0 {
            q.push_back((x / 2, d + 1));
        }
        q.push_back((x - 1, d + 1));
    }
    println!("Something bad happened");
    return;
}
