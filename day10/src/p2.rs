use good_lp::{
    constraint, default_solver, variable, variables, Expression, Solution, SolverModel,
};

struct Machine {
    btns: Vec<Vec<usize>>,
    joltages: Vec<u32>,
}

fn min_presses(machine: &Machine) -> u32 {
    let num_buttons = machine.btns.len();
    let num_counters = machine.joltages.len();

    // Build variable set: one integer variable per button, bounded [0, ∞).
    let mut vars = variables!();
    let xs: Vec<_> = (0..num_buttons)
        .map(|_| vars.add(variable().integer().min(0)))
        .collect();

    // Objective: minimise sum of all press counts.
    let objective: Expression = xs.iter().sum();
    let mut problem = vars.minimise(objective).using(default_solver);

    // One equality constraint per counter:
    //   sum of x_i over buttons i that feed counter j  =  target_j
    for j in 0..num_counters {
        let lhs: Expression = machine
            .btns
            .iter()
            .enumerate()
            .filter(|(_, b)| b.contains(&j))
            .map(|(i, _)| Expression::from(xs[i]))
            .sum();
        problem = problem.with(constraint!(lhs == machine.joltages[j] as f64));
    }

    let solution = problem.solve().expect("ILP should be feasible");

    xs.iter()
        .map(|&x| solution.value(x).round() as u32)
        .sum()
}

pub fn solve(input: &Vec<&str>) {
    let machines: Vec<Machine> = input
        .iter()
        .map(|line| {
            let mut parts: Vec<&str> = line.split(' ').collect();
            parts.remove(0); // drop indicator light diagram

            let btns: Vec<Vec<usize>> = parts
                .iter()
                .filter(|p| p.starts_with('('))
                .map(|s| {
                    s.trim_matches(|c| c == '(' || c == ')')
                        .split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect()
                })
                .collect();

            let joltages: Vec<u32> = parts
                .last()
                .unwrap()
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect();

            Machine { btns, joltages }
        })
        .collect();

    let total: u32 = machines.iter().map(min_presses).sum();
    println!("Total minimum presses: {}", total);
}