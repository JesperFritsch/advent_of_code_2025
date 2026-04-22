use std::collections::VecDeque;

pub const ADJACENT: [(isize, isize); 8] = [
    (-1, -1), ( 0, -1), ( 1, -1),
    (-1,  0),           ( 1,  0),
    (-1,  1), ( 0,  1), ( 1,  1)
];


pub struct Grid {
    grid: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize
}

pub struct GridAreaIter<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
    x_start: usize,
    x_end: usize,
    y_end: usize,
}


impl<'a> GridAreaIter<'a> {
    pub fn new(grid: &'a Grid, from: (usize, usize), to: (usize, usize)) -> Self {
        GridAreaIter {
            grid: grid,
            x: from.0,
            y: from.1,
            x_start: from.0,
            x_end: to.0,
            y_end: to.1
        }
    }
}


impl<'a> Iterator for GridAreaIter<'a> {
    type Item = (usize, usize, char);

    fn next(&mut self) -> Option<Self::Item> {

        if self.y >= self.y_end.min(self.grid.height) {
            return None;
        }

        let result = (
            self.x as usize,
            self.y as usize,
            self.grid.get(self.x, self.y).unwrap(),
        );
        self.x += 1;
        if self.x >= self.x_end.min(self.grid.width) {
            self.y += 1;
            self.x = self.x_start.max(0);
        }
        Some(result)
    }
}


impl Grid {

    pub fn new(grid: Vec<Vec<char>>) -> Self {
        let height = grid.len();
        let width = if height > 0 { grid[0].len() } else { 0 };
        Grid { grid, width, height }
    }

    pub fn from_dim(width: usize, height: usize, init: char) -> Self {
        let grid = vec![vec![init; width + 1 as usize]; height + 1 as usize];
        Grid { grid, width: width+1, height: height+1 }
    }

    pub fn fill_line(&mut self, from: (usize, usize), to: (usize, usize), s_gap: usize, e_gap: usize, ch: char) {
        if from.0 != to.0 && from.1 != to.1 {
            panic!("Not a straight line");
        }
        let x_start = from.0.min(to.0);
        let y_start = from.1.min(to.1);
        let horizontal = from.0 != to.0;
        let len = from.0.abs_diff(to.0).max(from.1.abs_diff(to.1));
        for i in s_gap..=len-e_gap {
            let mut set_x = x_start;
            let mut set_y = y_start;
            if horizontal {
                set_x = x_start + i;
            }
            else {
                set_y = y_start + i;
            }
            self.set(set_x, set_y, ch);
        }
    }

    pub fn fill_area(&mut self, start: (usize, usize), fill_value: char) {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let start_value = self.get(start.0, start.1).unwrap();
        queue.push_back(start);
        while let Some(curr) = queue.pop_front() {
            for (i, n) in self.neighbors(curr.0, curr.1).into_iter().enumerate() {
                if let Some(val) = n {
                    if val != start_value { continue; }
                    let nd = ADJACENT[i];
                    let nx = (curr.0 as isize + nd.0) as usize;
                    let ny = (curr.1 as isize + nd.1) as usize;
                    self.set(nx, ny, fill_value);
                    queue.push_back((nx, ny));
                } 
            }
        }
    }

    pub fn set_points(& mut self, points: &Vec<(usize, usize)>, ch: char) {
        for p in points.iter() {
            self.set(p.0, p.1, ch);
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        self.grid.get(y)?.get(x).copied()
    }

    pub fn set(& mut self, x: usize, y: usize, val: char) -> bool {
        if !self.get(x, y).is_some() {return false};
        self.grid[y as usize][x as usize] = val;
        true
    }

    pub fn count_neighbors(&self, x: usize, y: usize, target: char) -> usize {
        self.neighbors(x, y).iter().filter(|n| **n == Some(target)).count()
    }

    pub fn neighbors(&self, x: usize, y: usize) -> Vec<Option<char>> {
        ADJACENT.iter()
            .map(|(dx, dy)| {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                usize::try_from(nx).ok()
                    .zip(usize::try_from(ny).ok())
                    .and_then(|(ux, uy)| self.get(ux, uy))
            })
            .collect()
    }

    pub fn iter(&self) -> GridAreaIter<'_>{
        GridAreaIter::new(&self,(0, 0), (self.width, self.height))
    }

    pub fn area_iter(&self, from: (usize, usize), to: (usize, usize)) -> GridAreaIter<'_> {
        GridAreaIter::new(self, from, to)
    }

    pub fn as_string(&self, space: &str) -> String {
        self.iter()
            .map(|c| format!("{}{}", c.2, if c.0 == (self.width - 1) {"\n"} else {space} ))
            .collect()
    }

}