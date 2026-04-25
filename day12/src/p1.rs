use regex::Regex;

#[derive(Debug)]
struct Problem {
    width: usize,
    height: usize,
    shapes_to_use: Vec<u32>, // number of times to use each shape, index of number corresponds to index of shape.
}

pub fn solve(input: &Vec<&str>) {

    let shape_re = Regex::new(r"\d:$").unwrap();
    let problem_re = Regex::new(r"(\d{1,2})x(\d{1,2}): (.*)").unwrap();
    let mut shapes: Vec<Vec<Vec<bool>>> = vec!();
    let problems: Vec<Problem>; 
    let mut line_c: usize = 0;
    while shape_re.is_match(input[line_c]) {
        line_c += 1;
        let shape: Vec<Vec<bool>> = input[line_c..line_c+3].iter()
            .map(|l| l.chars()
                .map(|c| if c == '#' {true} else {false})
                .collect()
            )
            .collect();
        line_c += 4;
        shapes.push(shape);
    }
    problems = input[line_c..].iter()
    .map(|line| {
        let p_match = problem_re.captures(line).unwrap();
        let width = p_match[1].parse::<usize>().unwrap();
        let height = p_match[2].parse::<usize>().unwrap();
        let shapes_to_use = p_match[3].split(' ').map(|s| s.parse::<u32>().unwrap()).collect();
        Problem {width, height, shapes_to_use: shapes_to_use}
    }).collect();
    
    println!("shapes {:#?}", shapes);
    println!("problems {:#?}", problems);

}