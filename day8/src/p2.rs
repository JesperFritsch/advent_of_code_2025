
use std::collections::HashSet;

fn dist(p1: &Vec<i64>, p2: &Vec<i64>) -> f64{
    (((p1[0] - p2[0]).pow(2) +
     (p1[1] - p2[1]).pow(2) + 
     (p1[2] - p2[2]).pow(2)) as f64).sqrt()
}

pub fn solve(input: &Vec<&str>) {
    let mut circuits: Vec<HashSet<usize>> = vec!();
    let points: Vec<Vec<i64>> = input.iter()
        .map(|l| l.split(',').map(|d| d.parse::<i64>().unwrap()).collect())
        .collect();


    let mut pairs: Vec<(usize, usize, f64)> = (0..points.len())
        .flat_map(|i| {
            let points = &points;
            (i + 1..points.len()).map(move |j| {
                (i, j, dist(&points[i], &points[j]))
            })
        })
        .collect();

    pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    for pair in pairs.iter(){
        let idx_a = circuits.iter().position(|c| c.contains(&pair.0));
        let idx_b = circuits.iter().position(|c| c.contains(&pair.1));
        match (idx_a, idx_b) {
            (Some(a), Some(b)) if a != b => {
                let set_b = circuits.remove(b);
                let new_a_idx = if a < b { a } else { a - 1 };
                circuits[new_a_idx].extend(set_b);
            },
            (Some(_), Some(_)) => {}
            (Some(a), Option::None) => {
                circuits[a].insert(pair.1);
            }
            (Option::None, Some(b)) => {
                circuits[b].insert(pair.0);
            }
            _ => {
                circuits.push(HashSet::from([pair.0, pair.1]));
            }
        }
        if circuits.len() == 1 && circuits[0].len() == points.len() {
            let coord_a = &points[pair.0];
            let coord_b = &points[pair.1];
            println!("p2: {}", coord_a[0] * coord_b[0]);
            break;
        }
    }
}