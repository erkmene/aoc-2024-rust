fn parse_input_file(file: &str) -> Vec<Vec<i32>> {
    let contents = std::fs::read_to_string(file).unwrap_or_else(|_| {
        println!("ERROR: Input file not found.");
        std::process::exit(1);
    });
    let lines = contents.lines();
    lines
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let mut direction: i32 = 0;
    for i in 0..report.len() - 1 {
        let diff = report[i + 1] - report[i];
        if diff == 0 {
            // Need to bail out before dividing zero
            return false;
        }
        let diff_abs = diff.abs();
        let next_direction = diff / diff_abs;
        let check =
            (diff_abs > 0 && diff_abs < 4) && (direction == 0 || direction == next_direction);
        if check {
            direction = next_direction;
        } else {
            return false;
        }
    }
    true
}

fn generate_subsets(report: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut subsets: Vec<Vec<i32>> = Vec::new();
    for i in 0..report.len() {
        subsets.push(
            report
                .iter()
                .enumerate()
                .filter(|&(pos, _)| pos != i)
                .map(|(_, val)| val.clone())
                .collect(),
        )
    }
    subsets
}

fn check_reports(reports: &Vec<Vec<i32>>, with_subsets: bool) -> i32 {
    let mut safe_count = 0;
    for i in 0..reports.len() {
        let report = &reports[i];
        if is_report_safe(report) {
            safe_count += 1;
            continue;
        }
        if with_subsets {
            let subsets = generate_subsets(report);
            for j in 0..subsets.len() {
                if is_report_safe(&subsets[j]) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }
    safe_count
}

fn main() {
    let test_data = parse_input_file("02.test.dat");
    assert!(check_reports(&test_data, false) == 2);
    assert!(check_reports(&test_data, true) == 4);
    let data = parse_input_file("02.dat");
    println!("First answer: {}", check_reports(&data, false));
    println!("Second answer: {}", check_reports(&data, true));
}
