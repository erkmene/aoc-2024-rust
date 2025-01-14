fn parse_input_file(file: &str) -> Map {
    let contents = std::fs::read_to_string(file).unwrap_or_else(|_| {
        println!("ERROR: Input file not found.");
        std::process::exit(1);
    });
    contents
        .lines()
        .map(|line| line.chars().map(String::from).collect())
        .collect()
}

type Map = Vec<Vec<String>>;

fn find_char_in_map(map: &Map, needle: &str) -> (i32, i32) {
    let mut result: (i32, i32) = (0, 0);
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == needle {
                result = (row as i32, col as i32)
            }
        }
    }
    result
}

fn traverse_map(source_map: &Map) -> (Map, bool) {
    let mut clone: Map = source_map.clone();
    let map: &mut Map = clone.as_mut();

    let width = map.len() as i32;
    let height = map[0].len() as i32;

    let directions: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut current_dir_index = 0;
    let mut current_dir = directions[current_dir_index];
    let mut current_pos = find_char_in_map(&map, "^");
    let mut looped = false;
    loop {
        let new_pos = (current_pos.0 + current_dir.0, current_pos.1 + current_dir.1);
        if new_pos.0 >= height || new_pos.1 >= width || new_pos.0 < 0 || new_pos.1 < 0 {
            break;
        }
        let val = map[new_pos.0 as usize][new_pos.1 as usize].as_ref();
        match val {
            "#" => {
                current_dir_index = (current_dir_index + 1) % 4;
                current_dir = directions[current_dir_index];
                // println!("Turn {:?}", current_dir);
            }
            "0" | "1" | "2" | "3" => {
                current_pos = new_pos;
                if val == current_dir_index.to_string() {
                    looped = true;
                    break;
                }
            }
            // "." | "^" |
            _ => {
                current_pos = new_pos;
                if val != "^" {
                    map[current_pos.0 as usize][current_pos.1 as usize] =
                        current_dir_index.to_string();
                }
            }
        }
    }
    (map.to_vec(), looped)
}

fn get_visited_positions(
    traversed_map: &Map,
    include_starting_position: bool,
) -> Vec<(usize, usize)> {
    let mut visited_positions = Vec::new();

    for row in 0..traversed_map.len() {
        for col in 0..traversed_map[row].len() {
            let cell = &traversed_map[row][col];
            if cell == "0"
                || cell == "1"
                || cell == "2"
                || cell == "3"
                || (include_starting_position && cell == "^")
            {
                visited_positions.push((row, col));
            }
        }
    }

    visited_positions
}

fn count_possible_loops(source_map: &Map) -> i32 {
    let mut count = 0;
    let (traversed_map, _) = traverse_map(&source_map);
    let visited_positions = get_visited_positions(&traversed_map, false);

    for pos in visited_positions {
        let mut clone: Map = source_map.clone();
        let map: &mut Map = clone.as_mut();
        map[pos.0][pos.1] = "#".to_string();
        let (_, looped) = traverse_map(&map);
        if looped {
            count += 1;
        }
    }

    count
}

fn main() {
    let test_data = parse_input_file("06.test.dat");
    let (path, _looped) = traverse_map(&test_data);
    assert!(get_visited_positions(&path, true).len() == 41);
    assert!(count_possible_loops(&test_data) == 6);

    let data = parse_input_file("06.dat");

    let (path, _looped) = traverse_map(&data);
    println!("First Answer: {}", get_visited_positions(&path, true).len());
    println!("Second Answer: {}", count_possible_loops(&data));
}
