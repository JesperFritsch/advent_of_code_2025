
use std::collections::HashMap;

fn recurse_paths<'a>(
    connections: &HashMap<&'a str, Vec<&'a str>>,
    node: &'a str,
    target: &str,
    cache: &mut HashMap<&'a str, usize>
) -> usize {
    if let Some(&res) = cache.get(node) {
        return res;
    }
    if let Some(outs) = connections.get(node) {
        if outs.contains(&target) {
            return 1;
        }
        let result = outs.iter()
            .map(|conn| recurse_paths(connections, conn, target, cache))
            .sum::<usize>();
        cache.insert(node, result);
        result
    } else {
        0
    }
}

pub fn solve(input: &Vec<&str>) {

    let connections: HashMap<&str, Vec<&str>> = input.iter()
        .map(|l| l.split_once(':').unwrap())
        .map(|(f, t)| (f.trim(), t.split(' ').filter(|s| !s.is_empty()).collect())
        ).collect();


    let paths = |from, to| recurse_paths(&connections, from, to, &mut HashMap::new());

    let nr_paths =
        paths("svr", "fft") * paths("fft", "dac") * paths("dac", "out") +
        paths("svr", "dac") * paths("dac", "fft") * paths("fft", "out");

    println!("p2: {nr_paths}");

}