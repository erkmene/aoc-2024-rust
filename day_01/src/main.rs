use std::collections::HashMap;

fn parse_input_file(file: &str) -> [Vec<u32>; 2] {
    let contents = std::fs::read_to_string(file).unwrap_or_else(|_| {
        println!("ERROR: Input file not found \"{}\".", file);
        std::process::exit(1);
    });
    let lines = contents.lines();
    let mut columns: [Vec<u32>; 2] = [Vec::new(), Vec::new()];
    lines.for_each(|line| {
        let vals: Vec<&str> = line.split_whitespace().collect();
        for x in 0..2 {
            columns[x].push(vals[x].parse::<u32>().unwrap());
        }
    });
    return columns;
}

fn build_histogram(list: &Vec<u32>) -> HashMap<u32, u32> {
    let mut histogram: HashMap<u32, u32> = HashMap::new();
    for num in list.iter() {
        let item = histogram.entry(*num).or_insert(0);
        *item += 1;
    }
    return histogram;
}

fn calculate_distance(tuple: &[Vec<u32>; 2]) -> u32 {
    let mut sorted = tuple.clone();
    for i in 0..sorted.len() {
        sorted[i].sort();
    }
    let mut sum: u32 = 0;
    for i in 0..sorted[0].len() {
        sum += (sorted[1][i] as i32 - sorted[0][i] as i32).abs() as u32;
    }
    return sum;
}

fn calculate_similarity_scores(tuple: &[Vec<u32>; 2]) -> u32 {
    let histogram = build_histogram(&tuple[1]);
    let mut sum: u32 = 0;
    for num in tuple[0].iter() {
        sum += num * histogram.get(num).cloned().unwrap_or_else(|| 0);
    }
    return sum;
}

fn main() {
    let test_data = parse_input_file("01.test.dat");
    assert!(calculate_distance(&test_data) == 11);
    assert!(calculate_similarity_scores(&test_data) == 31);
    let data = parse_input_file("01.dat");
    println!("First Answer: {}", calculate_distance(&data));
    println!("Second Answer: {}", calculate_similarity_scores(&data));
}
