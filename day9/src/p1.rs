

fn calc_rec_area(p1: (u64, u64), p2: (u64, u64)) -> u64 {
    (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1)
}


pub fn solve(input: &Vec<&str>) {
    let coords: Vec<(u64, u64)> = input.iter().map(
        |l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
        }
    ).collect();

    let result: u64 = (0..coords.len())
        .flat_map(|i| {
            let coords = &coords;
            (i+1..coords.len())
            .map(move |j| 
                calc_rec_area(coords[i], coords[j])
            )}
        ).max().unwrap();
    
    println!("p1: {}", result)
}