use std::ops::RangeInclusive;


fn merge_ranges(mut ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    ranges.sort_by_key(|r| *r.start());
    let mut merged: Vec<RangeInclusive<u64>> = vec![];

    for range in ranges {
        if let Some(last) = merged.last_mut() {
            if range.start() <= &(*last.end() + 1) {
                *last = *last.start()..=*last.end().max(range.end());
            } else {
                merged.push(range);
            }
        } else {
            merged.push(range);
        }
    }
    merged
}


pub fn solve(input: &Vec<&str>) {
    let ranges_lines = input.split(|line| line.is_empty()).next().unwrap().to_vec();

    let ranges: Vec<RangeInclusive<u64>> = ranges_lines.iter()
        .map(|l| {
            let (a, b) = l.split_once('-').unwrap();
            a.parse::<u64>().unwrap()..=b.parse::<u64>().unwrap()
        })
        .collect();

    let ranges = merge_ranges(ranges);

    let count: u64 = ranges.iter()
        .map(|r| r.end() - r.start() + 1)
        .sum();

    println!("{}", count);
}