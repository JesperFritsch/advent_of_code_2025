use regex::Regex;
use std::time::Instant;

#[derive(Debug)]
struct Problem {
    width: usize,
    height: usize,
    shapes_to_use: Vec<usize>, // number of times to use each shape, index of number corresponds to index of shape.
}


#[derive(Debug)]
struct Node {
    u: usize,
    r: usize,
    d: usize,
    l: usize,
    c: usize,
    s: usize
}


#[derive(Clone, PartialEq, Eq, Hash)]
struct Shape {
    width: usize,
    height: usize,
    occupies: Vec<usize>, // sorted, canonical
}

impl Shape {

    fn rotate90(&self) -> Shape {
        // (x, y) -> (height - 1 - y, x); new dims = (height, width)
        let new_w = self.height;
        let mut occupies: Vec<usize> = self.occupies.iter().map(|&i| {
            let (x, y) = (i % self.width, i / self.width);
            let (nx, ny) = (self.height - 1 - y, x);
            ny * new_w + nx
        }).collect();
        occupies.sort_unstable();
        Shape { width: self.height, height: self.width, occupies }
    }

    fn flip_h(&self) -> Shape {
        let mut occupies: Vec<usize> = self.occupies.iter().map(|&i| {
            let (x, y) = (i % self.width, i / self.width);
            y * self.width + (self.width - 1 - x)
        }).collect();
        occupies.sort_unstable();
        Shape { width: self.width, height: self.height, occupies }
    }

    fn orientations(&self) -> Vec<Shape> {
        let mut seen = std::collections::HashSet::new();
        let mut out = Vec::new();
        let mut cur = self.clone();
        for _ in 0..4 {
            for s in [cur.clone(), cur.flip_h()] {
                if seen.insert(s.clone()) { out.push(s); }
            }
            cur = cur.rotate90();
        }
        out
    }

    fn render(&self) -> String {
        let mut repr = String::with_capacity((self.width + 1) * self.height);
        for y in 0..self.height{
            for x in 0..self.width {
                repr.push(if self.occupies.contains(&(y * self.width + x)) {'#'} else {' '});
            }
            repr.push('\n');
        }
        repr
    }

}


impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}


impl std::fmt::Debug for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\n{}", self.render())
    }
}


fn splice_v(arena: &mut [Node], before: usize, new: usize) {
    // inserts the new Node before the given "before" Node.
    let bb = arena[before].u;
    arena[bb].d = new;
    arena[new].u = bb;
    arena[new].d = before;
    arena[before].u = new;
}


fn splice_h(arena: &mut [Node], before: usize, new: usize) {
    // inserts the new Node before the given "before" Node.
    let bb = arena[before].l;
    arena[bb].r = new;
    arena[new].l = bb;
    arena[new].r = before;
    arena[before].l = new;
}


fn unlink_v(arena: &mut [Node], node: usize) {
    let before = arena[node].u;
    let after = arena[node].d;
    arena[before].d = after;
    arena[after].u = before;
}


fn relink_v(arena: &mut [Node], node: usize) {
    let before = arena[node].u;
    let after = arena[node].d;
    arena[before].d = node;
    arena[after].u = node;
}


fn unlink_h(arena: &mut [Node], node: usize) {
    let before = arena[node].l;
    let after = arena[node].r;
    arena[before].r = after;
    arena[after].l = before;
}


fn relink_h(arena: &mut [Node], node: usize) {
    let before = arena[node].l;
    let after = arena[node].r;
    arena[before].r = node;
    arena[after].l = node;
}


fn add_to_col(arena: &mut [Node], col: usize, node: usize) {
    splice_v(arena, col, node);
    arena[col].s += 1;
    arena[node].c = col;
}


fn ul_from_col(arena: &mut [Node], node: usize) {
    unlink_v(arena, node);
    let col = arena[node].c;
    arena[col].s -= 1;
}


fn rl_to_col(arena: &mut [Node], node: usize) {
    relink_v(arena, node);
    let col = arena[node].c;
    arena[col].s += 1;
}


fn get_smallest_col(arena: &[Node]) -> usize {
    let mut best = arena[0].r;        
    let mut best_s = arena[best].s;   
    let mut c = arena[best].r;        
    while c != 0 {                    
        if arena[c].s < best_s {
            best = c;
            best_s = arena[c].s;
        }
        c = arena[c].r;
    }
    best
}


fn cover(arena: &mut [Node], col: usize) {
    unlink_h(arena, col);
    let mut row = arena[col].d;
    while row != col {
        let mut r_col = arena[row].r;
        while r_col != row {
            ul_from_col(arena, r_col);
            r_col = arena[r_col].r;
        }
        row = arena[row].d;
    }
}


fn uncover(arena: &mut [Node], col: usize) {
    let mut row = arena[col].u;
    while row != col {
        let mut r_col = arena[row].l;
        while r_col != row {
            rl_to_col(arena, r_col);
            r_col = arena[r_col].l;
        }
        row = arena[row].u;
    }
    relink_h(arena, col);
}


fn first_cell_position(arena: &[Node], row: usize, secondary_first: usize) -> usize {
    let first_cell_node = arena[row].r;
    arena[first_cell_node].c - secondary_first
}


fn search(
    arena: &mut [Node],
    solution: &mut Vec<usize>,
    instance_to_shape: &[usize],
    shape_to_instances: &[Vec<usize>],
    selected_position: &mut [Option<usize>],
    primary_first: usize,
    secondary_first: usize,
) -> bool {
    if arena[0].r == 0 {
        return true;
    }
    let col = get_smallest_col(arena);
    if arena[col].s == 0 {
        return false;
    }
    cover(arena, col);
    let mut row = arena[col].d;
    while row != col {
        let this_i_idx = col - primary_first;
        let this_i_pos = first_cell_position(arena, row, secondary_first);
        let shape_idx = instance_to_shape[this_i_idx];
        let instance_idxs = &shape_to_instances[shape_idx];
        let mut ok = true;

        // Prune when an instance has a lower index and higher position
        for &i_idx in instance_idxs.iter() {
            if i_idx == this_i_idx { continue; }
            if let Some(i_pos) = selected_position[i_idx] {
                let same_order_by_idx = this_i_idx < i_idx;
                let same_order_by_pos = this_i_pos < i_pos;
                if same_order_by_idx != same_order_by_pos {
                    ok = false;
                    break;
                }
            }
        }

        if !ok {
            row = arena[row].d;
            continue;
        }

        solution.push(row);

        selected_position[this_i_idx] = Some(this_i_pos);

        let mut r_col = arena[row].r;
        while r_col != row {
            cover(arena, arena[r_col].c);
            r_col = arena[r_col].r;
        }

        let mut dead = false;
        let mut c = arena[0].r;
        while c != 0 {
            if arena[c].s == 0 {
                dead = true;
                break;
            }
            c = arena[c].r;
        }

        if !dead {
            if search(
                arena, 
                solution,
                instance_to_shape,
                shape_to_instances,
                selected_position,
                primary_first,
                secondary_first
            ) {
                return true;
            }
        }

        solution.pop();

        selected_position[this_i_idx] = None;
        
        r_col = arena[row].l;
        while r_col != row {
            uncover(arena, arena[r_col].c);
            r_col = arena[r_col].l;
        }

        row = arena[row].d;
    }
    uncover(arena, col);
    false
}

 
fn matrix_builder(
    shapes: &Vec<Vec<Shape>>, 
    problem: &Problem, 
    arena: &mut Vec<Node>,
    instance_to_shape: &mut Vec<usize>,
    shape_to_instances: &mut Vec<Vec<usize>>,
    selected_position: &mut Vec<Option<usize>>,
    primary_first: &mut usize,
    secondary_first: &mut usize,
){
    let root_node = Node {u: 0, r: 0, d: 0, l: 0, c: 0, s: 0};
    arena.push(root_node);
    // Build the column headers
    let total_instances: usize = problem.shapes_to_use.iter().sum();
    shape_to_instances.resize(shapes.len(), Vec::new());
    for _ in 0..total_instances {
        let node_idx = arena.len();
        let node = Node {
            u: node_idx,
            r: node_idx,
            d: node_idx,
            l: node_idx,
            c: node_idx,
            s: 0,
        };
        arena.push(node);
        let last = arena.len() - 1;
        splice_h(arena, 0, last);
    }
    *primary_first = 1;
    *secondary_first = arena.len(); 
    for _ in 0..(problem.height * problem.width) {
        let node_idx = arena.len();
        let node = Node {
            u: node_idx,
            r: node_idx,
            d: node_idx,
            l: node_idx,
            c: node_idx,
            s: 0,
        };
        arena.push(node);
    }

    // Build the rows

    for (instance_idx, shape_idx) in problem.shapes_to_use.iter()
        .enumerate()
        .flat_map(|(i, &nr)| std::iter::repeat_n(i, nr)
    ).enumerate(){ // iterate over each shape index as many times as the problem needs the shape to fit
        instance_to_shape.push(shape_idx);
        selected_position.push(None);
        shape_to_instances[shape_idx].push(instance_idx);
        for s_orient in shapes[shape_idx].iter() {
            if s_orient.height > problem.height || s_orient.width > problem.width {
                continue;
            }
            let s_grid_coords: Vec<(usize, usize)> = s_orient.occupies.iter()
                .map(|s_idx| (s_idx % s_orient.width, s_idx / s_orient.width))
                .collect();
            let grid_coords = (0..=(problem.height - s_orient.height))
                .flat_map(|y| (0..=(problem.width - s_orient.width))
                .map(move |x| (x, y))
            );
            for (x, y) in grid_coords {
                let inst_node_idx = arena.len();
                // create a node for the specific shape instance
                let inst_coll = *primary_first + instance_idx;
                let inst_node = Node {
                    u: inst_node_idx,
                    r: inst_node_idx,
                    d: inst_node_idx,
                    l: inst_node_idx,
                    c: 0,
                    s: 0
                };
                arena.push(inst_node);
                // add as a new row
                add_to_col(arena, inst_coll, inst_node_idx);
                // create occupancy nodes for the coordinates this instance in this orientation occupies
                for (sx, sy) in s_grid_coords.iter() {
                    let ox = x + sx;
                    let oy = y + sy;
                    let grid_idx = oy * problem.width + ox;
                    let o_node_idx = arena.len(); 
                    let coll_idx = *secondary_first + grid_idx;
                        let o_node = Node {
                        u: o_node_idx,
                        r: o_node_idx,
                        d: o_node_idx,
                        l: o_node_idx,
                        c: 0, // assigned by add_to_col
                        s: 0
                    };
                    arena.push(o_node);
                    // add occupancy node to the same row as the instance node
                    splice_h(arena, inst_node_idx, o_node_idx);
                    // connect to the column for this grid index
                    add_to_col(arena, coll_idx, o_node_idx);
                }
            }
        }
    }
}


fn easy_check(problem: &Problem, shapes: &Vec<Vec<Shape>>) -> i32 {
// returns 1 if all shapes fit naively on the grid
// returns -1 if total occupies cells by shapes is greater than the grid ares
// returns 0 otherwise
    let shape_areas: Vec<usize> = shapes.iter()
        .map(|s| s[0].height * s[0].width)
        .collect();
    let grid_area = problem.height * problem.width;
    let total_shapes_area: usize = shape_areas.iter()
        .enumerate()
        .fold(0, |acc, (i, a)| acc + (a * problem.shapes_to_use[i]));

    if total_shapes_area <= grid_area{
        return 1;
    }

    let total_occupied = shapes.iter()
        .enumerate()
        .fold(0, |acc, (i, s)| acc + (problem.shapes_to_use[i] * s[0].occupies.len()));

    if total_occupied > grid_area {
        return -1;
    }
    0
}


pub fn solve(input: &Vec<&str>) {

    let shape_re = Regex::new(r"\d:$").unwrap();
    let problem_re = Regex::new(r"(\d{1,2})x(\d{1,2}): (.*)").unwrap();
    let mut shapes: Vec<Vec<Shape>> = vec!();
    let problems: Vec<Problem>; 
    let mut line_c: usize = 0;
    while shape_re.is_match(input[line_c]) {
        line_c += 1;
        let width = input[line_c].len();
        let mut occupies: Vec<usize> = vec!();
        let mut height = 0;
        while input[line_c].len() == width {
            for (i, char) in input[line_c].chars().enumerate() {
                if char == '#' {
                    occupies.push(i + (width * height));
                }
            }
            height += 1;
            line_c += 1;
        }
        line_c += 1;
        let shape = Shape {width, height, occupies};
        shapes.push(shape.orientations());
    }
    problems = input[line_c..].iter()
    .map(|line| {
        let p_match = problem_re.captures(line).unwrap();
        let width = p_match[1].parse::<usize>().unwrap();
        let height = p_match[2].parse::<usize>().unwrap();
        let shapes_to_use = p_match[3].split(' ').map(|s| s.parse::<usize>().unwrap()).collect();
        Problem {width, height, shapes_to_use: shapes_to_use}
    }).collect();
    
    let mut arena: Vec<Node> = vec!();
    let mut instance_to_shape: Vec<usize> = vec!();
    let mut shape_to_instances: Vec<Vec<usize>> = vec!();
    let mut selected_position: Vec<Option<usize>> = vec!();
    let mut primary_first: usize = 0;
    let mut secondary_first: usize = 0;

    let mut solvable = 0;
    for problem in problems.iter() {
        let solved;
        match easy_check(problem, &shapes) {
            -1 => solved = false,
            1 => solved = true,
            _ => {
                arena.clear();
                instance_to_shape.clear();
                selected_position.clear();
                shape_to_instances.clear();
                matrix_builder(
                    &shapes, 
                    problem, 
                    &mut arena,
                    &mut instance_to_shape,
                    &mut shape_to_instances,
                    &mut selected_position,
                    &mut primary_first,
                    &mut secondary_first,
                );
                solved = search(
                    &mut arena, 
                    &mut vec!(),
                    &instance_to_shape,
                    &shape_to_instances,
                    &mut selected_position,
                    primary_first,
                    secondary_first,
                );
            }
        }
        if solved { solvable += 1; }

    }

    println!("solvable {}", solvable);

}