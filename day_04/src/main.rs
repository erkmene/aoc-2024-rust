fn parse_input_file(file: &str) -> Vec<Vec<String>> {
    std::fs::read_to_string(file)
        .unwrap_or_else(|_| {
            println!("ERROR: Input file not found.");
            std::process::exit(1);
        })
        .lines()
        .map(|line| line.chars().map(|char| char.to_string()).collect())
        .collect()
}

fn get_val_at_coord(grid: &Vec<Vec<String>>, r: usize, c: usize) -> String {
    let row_size = grid.len();
    let col_size = grid[0].len();

    if r < row_size && c < col_size {
        grid[r][c].clone()
    } else {
        "".to_string()
    }
}

fn count_xmas(grid: &Vec<Vec<String>>) -> i32 {
    let row_size = grid.len();
    let col_size = grid[0].len();

    let mut sum = 0;

    let range_size = 4;
    for r in 0..row_size {
        for c in 0..col_size {
            let slices: [String; 4] = [
                (0..range_size)
                    .map(|i| get_val_at_coord(&grid, r, c + i))
                    .collect(),
                (0..range_size)
                    .map(|i| get_val_at_coord(&grid, r + i, c))
                    .collect(),
                (0..range_size)
                    .map(|i| get_val_at_coord(&grid, r + i, c + i))
                    .collect(),
                (0..range_size)
                    .map(|i| get_val_at_coord(&grid, r + i, c + range_size - i - 1))
                    .collect(),
            ];
            slices.map(|slice: String| match slice == "XMAS" || slice == "SAMX" {
                true => sum = sum + 1,
                false => (),
            });
        }
    }
    sum
}

fn count_x_mas(grid: &Vec<Vec<String>>) -> i32 {
    let row_size = grid.len();
    let col_size = grid[0].len();

    let mut sum = 0;

    let range_size = 3;
    for r in 0..row_size {
        for c in 0..col_size {
            let slices: [String; 2] = [
                (0..range_size)
                    .map(|i| get_val_at_coord(&grid, r + i, c + i))
                    .collect(),
                (0..range_size)
                    .map(|i| get_val_at_coord(&grid, r + i, c + range_size - i - 1))
                    .collect(),
            ];
            let mut match_count = 0;
            slices.map(|slice: String| match slice == "SAM" || slice == "MAS" {
                true => {
                    match_count += 1;
                }
                false => (),
            });
            if match_count == 2 {
                sum = sum + 1;
            }
        }
    }
    sum
}

fn main() {
    let test_data = parse_input_file("04.test.dat");
    assert!(count_xmas(&test_data) == 18);
    assert!(count_x_mas(&test_data) == 9);
    let data = parse_input_file("04.dat");
    println!("First answer: {}", count_xmas(&data));
    println!("Second answer: {}", count_x_mas(&data));
}
