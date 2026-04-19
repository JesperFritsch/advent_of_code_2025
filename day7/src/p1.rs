

pub fn solve(input: &Vec<&str>) {
    let mut beams: Vec<bool> = vec![false; input[0].len()];
    beams[input[0].chars().position(|c| c == 'S').unwrap()] = true;
    let mut total_splits = 0;
    for plane in input.iter() {

        let split_idxs: Vec<usize> = plane.char_indices()
            .filter(|(_, c)| *c == '^')
            .map(|(i, _)| i)
            .collect();

        total_splits += beams.iter()
            .enumerate()
            .filter(|(i, b)| **b && split_idxs.contains(i))
            .count();
        
        for &i in split_idxs.iter() {
            if beams[i] {
                beams[i] = false;
                beams[i - 1] = true;
                beams[i + 1] = true;
            }
        }
    }
    println!("{}", total_splits);
}