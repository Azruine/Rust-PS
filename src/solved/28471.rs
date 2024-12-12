fn main() {
    let dir: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n: i32 = s.trim().parse().unwrap();
    let map: Vec<Vec<char>> = std::io::stdin()
        .lines()
        .take(n as usize)
        .map(|line| line.unwrap().trim().chars().collect())
        .collect();

    let start_pos: Vec<(i32, i32)> = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, &c)| {
                    if c == '.' {
                        Some((i as i32, j as i32))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .collect();

    let goal: (i32, i32) = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find(|&(_, &c)| c == 'F')
                .map(|(j, _)| (i as i32, j as i32))
        })
        .expect("Goal not found");
    let mut path_cnt = 0;
    let mut q = std::collections::VecDeque::new();
    q.push_back(goal);
    let mut visited = vec![vec![false; n as usize]; n as usize];
    visited[goal.0 as usize][goal.1 as usize] = true;
    while let Some((x, y)) = q.pop_front() {
        for (dx, dy) in &dir {
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0
                && nx < n
                && ny >= 0
                && ny < n
                && !visited[nx as usize][ny as usize]
                && map[nx as usize][ny as usize] != '#'
            {
                visited[nx as usize][ny as usize] = true;
                q.push_back((nx, ny));
            }
        }
    }
    for (i, j) in start_pos {
        if visited[i as usize][j as usize] {
            path_cnt += 1;
        }
    }
    println!("{}", path_cnt);
}
