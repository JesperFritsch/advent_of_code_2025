use common::Grid;
use std::ops::RangeInclusive;
use std::fs;

fn calc_rec_area(p1: (u64, u64), p2: (u64, u64)) -> u64 {
    (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1)
}


pub fn solve(input: &Vec<&str>) {
    let coords: Vec<(usize, usize)> = input.iter().map(
        |l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        }
    ).collect();

    let mut recs: Vec<((usize, usize), (usize, usize), u64)> = (0..coords.len())
        .flat_map(|i| {
            let coords = &coords;
            (i + 1..coords.len()).map(move |j| {
                let (x1, y1) = coords[i];
                let (x2, y2) = coords[j];
                (coords[i], coords[j], calc_rec_area((x1 as u64, y1 as u64), (x2 as u64, y2 as u64)))
            })
        })
        .collect();
    // (row/column, edge range)
    let mut ver_edges: Vec<(usize, RangeInclusive<usize>)> = vec!();
    let mut hor_edges: Vec<(usize, RangeInclusive<usize>)> = vec!();
    let extended_coords: Vec<(usize, usize)> = coords.iter().chain(std::iter::once(&coords[0])).copied().collect();

    for w in extended_coords.windows(2){
        let (a, b) = (w[0], w[1]);
        if a.0 == b.0 {
            ver_edges.push((a.0, a.1.min(b.1)+1..=a.1.max(b.1)-1));
        } else {
            hor_edges.push((a.1, a.0.min(b.0)+1..=a.0.max(b.0)-1));
        }
    }

    ver_edges.sort_by(|a, b| a.0.cmp(&b.0));
    hor_edges.sort_by(|a, b| a.0.cmp(&b.0));
    recs.sort_by(|a, b| b.2.cmp(&a.2));

    for rec in recs.iter() {
        let (p1, p2) = (rec.0, rec.1);
        let top_left = (p1.0.min(p2.0), p1.1.min(p2.1));
        let top_right = (p1.0.max(p2.0), p1.1.min(p2.1));
        let bot_left = (p1.0.min(p2.0), p1.1.max(p2.1));
        let bot_right = (p1.0.max(p2.0), p1.1.max(p2.1));
        let rec_edges_h = [
            (top_left.1, top_left.0..top_right.0),
            (bot_left.1, bot_left.0..bot_right.0),
        ];
        let rec_edges_v = [
            (top_left.0, top_left.1..bot_left.1),
            (top_right.0, top_right.1..bot_right.1),        
        ];
        if rec_edges_h.iter()
            .any(|re| ver_edges.iter().any(|ve| re.1.contains(&ve.0) && ve.1.contains(&re.0))){
                continue;
        } 
        if rec_edges_v.iter()
            .any(|re| hor_edges.iter().any(|he| re.1.contains(&he.0) && he.1.contains(&re.0))){
                continue;
        }
        
        println!("p2: {}", rec.2);
        break;
    }

    // let (xs, ys): (Vec<usize>, Vec<usize>) = coords.iter().map(|&(x, y)| (x, y)).unzip();
    // let max_x = *xs.iter().max().unwrap();
    // let max_y = *ys.iter().max().unwrap();
    
    // let mut grid = Grid::from_dim(max_x + 2, max_y + 2, '.');
    // let mut grid2 = Grid::from_dim(max_x + 2, max_y + 2, '.');

    // for edge in ver_edges.iter() {
    //     println!("{:?}", edge);
    //     grid.fill_line((edge.0, *edge.1.start()), (edge.0, *edge.1.end()), 0, 0, '#');
    // }
    // for edge in hor_edges.iter() {
    //     println!("{:?}", edge);
    //     grid2.fill_line((*edge.1.start(), edge.0), (*edge.1.end(), edge.0), 0, 0, '#');
    // }
    // fs::write("grid.txt", format!("{}\n\n{}", grid.as_string(""), grid2.as_string(""))).unwrap();



}