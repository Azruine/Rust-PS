use std::collections::HashMap;
use std::io::Write;

fn 다이나믹로동(dp: &mut HashMap<usize, i64>, n: usize) -> i64 {
    if let Some(&v) = dp.get(&n) {
        v
    } else {
        let v = (다이나믹로동(dp, n - 1) + 다이나믹로동(dp, n - 2)) % 10007;
        dp.insert(n, v);
        v
    }
}

fn main() {
    let stdout = std::io::stdout();
    let mut dp: HashMap<usize, i64> = HashMap::from([(0, 1), (1, 1)]);
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    let ans = 다이나믹로동(&mut dp, n);
    let mut out = std::io::BufWriter::new(stdout.lock());
    writeln!(out, "{}", ans).unwrap();
}
