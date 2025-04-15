use std::io::{BufRead, Write, stdin};

fn main() {
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    let mut lines = stdin().lock().lines();
    let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let mut arr: Vec<i32> = Vec::with_capacity(n);

    for line in lines.take(n) {
        let n = line.unwrap().trim().parse::<i32>().unwrap();
        arr.push(n);
    }
    arr.sort();
    let ans = arr
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    writeln!(out, "{}", ans).unwrap();
}
