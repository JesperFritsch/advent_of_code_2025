use std::collections::HashMap;


pub fn solve(input: &Vec<&str>) {

    let beam_idx = input[0].chars().position(|c| c == 'S').unwrap();
    let mut cache: HashMap<(usize, usize), u64> = HashMap::new();

    fn recurse(planes: &Vec<&str>, beam_idx: usize, depth: usize, cache: &mut HashMap<(usize, usize), u64>) -> u64{
        if let Some(&result) = cache.get(&(beam_idx, depth)) {
            return result;
        }
        if depth >= planes.len() {
            return 1;
        }
        let mut result = 0;
        if planes[depth].as_bytes()[beam_idx] == b'^' {
            result += recurse(planes, beam_idx + 1, depth + 1, cache);
            result += recurse(planes, beam_idx - 1, depth + 1, cache);
        }
        else {
            result += recurse(planes, beam_idx, depth + 1, cache);
        }
        cache.insert((beam_idx, depth), result);
        result
    }
    
    let total_timelines = recurse(input, beam_idx, 0, &mut cache);

    println!("{}", total_timelines);
}