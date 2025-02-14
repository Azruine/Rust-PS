struct Cuboid {
    x_lo: i64,
    x_hi: i64,
    y_lo: i64,
    y_hi: i64,
    z_lo: i64,
    z_hi: i64,
}

impl Cuboid {
    fn new() -> Cuboid {
        let mut temp: String = String::new();
        std::io::stdin().read_line(&mut temp).unwrap();
        let mut iter = temp.split_whitespace();
        let x_lo = iter.next().unwrap().parse().unwrap();
        let x_hi = iter.next().unwrap().parse().unwrap();
        let y_lo = iter.next().unwrap().parse().unwrap();
        let y_hi = iter.next().unwrap().parse().unwrap();
        let z_lo = iter.next().unwrap().parse().unwrap();
        let z_hi = iter.next().unwrap().parse().unwrap();

        Cuboid {
            x_lo,
            x_hi,
            y_lo,
            y_hi,
            z_lo,
            z_hi,
        }
    }
}
fn overlapped_area(a: &Cuboid, b: &Cuboid) -> bool {
    let x_overlap = std::cmp::max(
        0,
        std::cmp::min(a.x_hi, b.x_hi) - std::cmp::max(a.x_lo, b.x_lo),
    );
    let y_overlap = std::cmp::max(
        0,
        std::cmp::min(a.y_hi, b.y_hi) - std::cmp::max(a.y_lo, b.y_lo),
    );
    x_overlap > 0 && y_overlap > 0
}

fn main() {
    let arrow = Cuboid::new();
    let target = Cuboid::new();
    let mut time: i64 = 0;
    if overlapped_area(&arrow, &target) {
        if arrow.z_lo >= target.z_hi {
            time = arrow.z_lo - target.z_hi + 1;
        } else if arrow.z_lo <= target.z_lo && arrow.z_lo < target.z_hi {
            time = 0;
        } else if arrow.z_hi <= target.z_lo {
            time = -1;
        }
    } else {
        time = -1;
    }
    println!("{}", time);
}
