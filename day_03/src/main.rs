use regex::Regex;

fn parse_input_file(file: &str) -> String {
    std::fs::read_to_string(file).unwrap_or_else(|_| {
        println!("ERROR: Input file not found.");
        std::process::exit(1);
    })
}

#[derive(Debug)]
struct Operation {
    command: String,
    params: Option<[i32; 2]>,
}

fn extract_operations(instructions: &String) -> Vec<Operation> {
    let r = Regex::new(r"(mul\(([0-9]*),([0-9]*)\)|don't\(\)|do\(\))").unwrap();
    let operations: Vec<Operation> = r
        .captures_iter(instructions)
        .map(|mat| {
            let command = String::from(&mat[1]);
            let is_modifier_command = command == "don't()" || command == "do()";
            Operation {
                command: match is_modifier_command {
                    true => command.replace("()", ""),
                    false => String::from("mul"),
                },
                params: match is_modifier_command {
                    true => None,
                    false => Some([
                        mat[2].parse::<i32>().unwrap(),
                        mat[3].parse::<i32>().unwrap(),
                    ]),
                },
            }
        })
        .collect();
    operations
}

fn execute_operations(operations: &Vec<Operation>, with_extra: bool) -> i32 {
    let mut disabled = false;
    let mut sum = 0;
    operations.iter().for_each(|o| match o.command.as_ref() {
        "don't" | "do" => match with_extra {
            true => {
                disabled = o.command == "don't";
            }
            false => (),
        },
        "mul" => {
            if !disabled {
                let params = o.params.unwrap();
                sum += params[0] * params[1];
            }
        }
        _ => (),
    });
    sum
}

fn run_program(program: &String, with_extra: bool) -> i32 {
    execute_operations(&extract_operations(&program), with_extra)
}

fn main() {
    let test_data = parse_input_file("03.test.dat");
    assert!(run_program(&test_data, false) == 161);
    assert!(run_program(&test_data, true) == 48);
    let data = parse_input_file("03.dat");
    println!("First answer: {}", run_program(&data, false));
    println!("Second answer: {}", run_program(&data, true));
}
