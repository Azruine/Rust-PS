use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io::stdin;
use std::io::BufRead;
use std::io::Write;

fn main() {
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    let mut lines = stdin().lock().lines();

    let line = lines.next().unwrap().unwrap();

    let mut iter = line.split_whitespace();
    let v: i32 = iter.next().unwrap().parse().unwrap();

    let line = lines.next().unwrap().unwrap();
    let s: i32 = line.trim().parse().unwrap();

    let mut graph: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    for line in lines {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let u: i32 = iter.next().unwrap().parse().unwrap();
        let v: i32 = iter.next().unwrap().parse().unwrap();
        let w: i32 = iter.next().unwrap().parse().unwrap();
        graph.entry(u - 1).or_default().push((v - 1, w));
    }

    let mut pq = BinaryHeap::from([(Reverse(0), s - 1)]);

    let mut distances: Vec<Option<i32>> = vec![None; v as usize];
    distances[(s - 1) as usize] = Some(0);

    while let Some((Reverse(cur_distance), cur_node)) = pq.pop() {
        if let Some(d) = distances[cur_node as usize] {
            if d < cur_distance {
                continue;
            }
        }
        if let Some(neighbors) = graph.get(&cur_node) {
            for &(new_node, weight) in neighbors {
                let new_distance = cur_distance + weight;
                match distances[new_node as usize] {
                    Some(distance) if distance <= new_distance => continue,
                    _ => {
                        distances[new_node as usize] = Some(new_distance);
                        pq.push((Reverse(new_distance), new_node));
                    }
                }
            }
        }
    }

    for distance in distances {
        match distance {
            Some(d) => writeln!(out, "{}", d).unwrap(),
            None => writeln!(out, "INF").unwrap(),
        }
    }
}