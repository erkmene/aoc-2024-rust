use std::time::Instant;

#[derive(Debug, Clone)]
struct Operation {
    test: u64,
    numbers: Vec<u64>,
}

type Operations = Vec<Operation>;

fn parse_input_file(file: &str) -> Operations {
    let mut operations: Operations = Vec::new();
    let contents = std::fs::read_to_string(file).unwrap_or_else(|_| {
        println!("ERROR: Input file not found.");
        std::process::exit(1);
    });
    for line in contents.lines() {
        let split: Vec<&str> = line.split(": ").collect();
        operations.push(Operation {
            test: split[0].parse().unwrap(),
            numbers: split[1]
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        });
    }
    operations.to_vec()
}

// https://stackoverflow.com/questions/50277050/format-convert-a-number-to-a-string-in-any-base-including-bases-other-than-deci
fn format_radix(mut x: u64, radix: u32, pad: usize) -> String {
    let mut result = vec![];

    let mut remaining_pad = pad;

    loop {
        let m = x % radix as u64;
        x = x / radix as u64;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m as u32, radix).unwrap());
        remaining_pad -= 1;
        if x == 0 && remaining_pad <= 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}

fn test_operations(operations: &Operations, operator_count: u32) -> Vec<&Operation> {
    let mut correct_operations = Vec::new();

    let radix: u32 = operator_count;
    for operation in operations {
        let mut correct = false;
        let pow = operation.numbers.len() - 1;
        let permutations = radix.pow(pow as u32);
        'operations: for operator_permutation in 0..permutations as u32 {
            let mut result = operation.numbers[0];
            let bits = format_radix(operator_permutation as u64, radix, pow);
            let mut bit_mask = bits.chars();

            for number_index in 1..operation.numbers.len() {
                let number = operation.numbers[number_index];
                let operator = bit_mask.next().unwrap().to_string();
                // e.g. 001010 becomes *,*,+,*,+,*
                result = match operator.as_str() {
                    "0" => result * number,
                    "1" => result + number,
                    "2" => [result.to_string(), number.to_string()]
                        .join("")
                        .parse()
                        .unwrap(),
                    _ => panic!("Unexpected operator {}", operator),
                };
            }
            if result == operation.test {
                correct = true;
                break 'operations;
            }
        }
        if correct {
            correct_operations.push(operation);
        }
    }
    correct_operations
}

fn sum_test_values(operations: &Vec<&Operation>) -> u64 {
    operations.iter().fold(0, |acc, o| acc + o.test)
}

fn main() {
    let test_data: Vec<Operation> = parse_input_file("07.test.dat");
    let filtered = test_operations(&test_data, 2);
    assert!(filtered.len() == 3);
    assert!(sum_test_values(&filtered) == 3749);
    let filtered = test_operations(&test_data, 3);
    assert!(filtered.len() == 6);
    assert!(sum_test_values(&filtered) == 11387);

    let data: Vec<Operation> = parse_input_file("07.dat");
    let now = Instant::now();
    let filtered = test_operations(&data, 2);
    println!("First answer: {}", sum_test_values(&filtered));
    println!("Elapsed: {:.2?}", now.elapsed());
    let now = Instant::now();
    let filtered = test_operations(&data, 3);
    println!("Second answer: {}", sum_test_values(&filtered));
    println!("Elapsed: {:.2?}", now.elapsed());
}
