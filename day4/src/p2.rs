
use common::Grid;


pub fn solve(input: &Vec<&str>) {
    let grid_vec: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let mut grid = Grid::new(grid_vec);

    let mut sum: u32 = 0;
    let mut to_remove: Vec<(usize, usize)> = vec!();
    loop {

        let roll_coords = grid.iter()
            .filter(|(_, _, c)| *c == '@')
            .map(|(x, y, _)| (x, y));
            
        for (x, y) in roll_coords {
            if grid.count_neighbors(x, y, '@') < 4 {
                to_remove.push((x, y));
            }
        }
        if !to_remove.is_empty() {
            sum += to_remove.len() as u32;
            for (x, y) in to_remove.iter() {
                grid.set(*x, *y, '.');
            }
            to_remove.clear();
        } else {
            break;
        }
    }
    println!("{}", sum);
}