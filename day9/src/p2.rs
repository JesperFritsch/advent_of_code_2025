
fn calc_rec_area(p1: (i64, i64), p2: (i64, i64)) -> u64 {
    ((p1.0 - p2.0).abs() + 1) as u64 * ((p1.1 - p2.1).abs() + 1) as u64
}

pub fn solve(input: &Vec<&str>) {
    let coords: Vec<(i64, i64)> = input.iter().map(
        |l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
        }
    ).collect();

    let mut edges: Vec<(i64, i64, i64, i64)> = vec![];
    for w in coords.windows(2) {
        edges.push((w[0].0, w[0].1, w[1].0, w[1].1));
    }
    edges.push((coords.last().unwrap().0, coords.last().unwrap().1, coords[0].0, coords[0].1));

    let intersects = |min_x: i64, min_y: i64, max_x: i64, max_y: i64| -> bool {
        edges.iter().any(|e| {
            let (e_min_x, e_max_x) = (e.0.min(e.2), e.0.max(e.2));
            let (e_min_y, e_max_y) = (e.1.min(e.3), e.1.max(e.3));
            min_x < e_max_x && max_x > e_min_x && min_y < e_max_y && max_y > e_min_y
        })
    };

    let mut result: u64 = 0;
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let (a, b) = (coords[i], coords[j]);
            let min_x = a.0.min(b.0);
            let max_x = a.0.max(b.0);
            let min_y = a.1.min(b.1);
            let max_y = a.1.max(b.1);
            let manhattan = (a.0 - b.0).abs() + (a.1 - b.1).abs();
            if (manhattan * manhattan) as u64 > result {
                if !intersects(min_x, min_y, max_x, max_y) {
                    let area = calc_rec_area(a, b);
                    if area > result {
                        result = area;
                    }
                }
            }
            
        }
    }
    println!("p2: {}", result);
}