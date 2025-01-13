use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

#[derive(Debug)]
pub struct Input {
    pub rules: Vec<(i32, i32)>,
    pub updates: Vec<Vec<i32>>,
    pub after_lookup: HashMap<i32, HashSet<i32>>,
}

fn parse_input_file(file: &str) -> Input {
    let contents = std::fs::read_to_string(file).unwrap_or_else(|_| {
        println!("ERROR: Input file not found.");
        std::process::exit(1);
    });

    let sections: Vec<&str> = contents.split("\n\n").collect();

    let rules: Vec<(i32, i32)> = sections[0]
        .lines()
        .map(|line| {
            let mut parts = line
                .split("|")
                .map(|str| str.to_string().parse::<i32>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect();

    let updates: Vec<Vec<i32>> = sections[1]
        .lines()
        .map(|line| {
            line.split(",")
                .map(|str| str.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut after_lookup: HashMap<i32, HashSet<i32>> = HashMap::new();

    for rule_tuple in &rules {
        let (before, after) = rule_tuple;
        let rule = after_lookup.entry(*before).or_insert(HashSet::new());
        rule.insert(*after);
    }

    Input {
        rules,
        updates,
        after_lookup,
    }
}

struct SplitUpdates {
    correct: Vec<Vec<i32>>,
    incorrect: Vec<Vec<i32>>,
}

fn split_updates(input: &Input) -> SplitUpdates {
    let mut correct: Vec<Vec<i32>> = Vec::new();
    let mut incorrect: Vec<Vec<i32>> = Vec::new();

    let after_lookup = &input.after_lookup;

    for update in &input.updates {
        let is_correct: bool = update.iter().enumerate().fold(true, |acc, (index, page)| {
            return acc
                && ((index == 0)
                    || after_lookup
                        .get(&update[index - 1])
                        .unwrap_or(&HashSet::new())
                        .contains(page));
        });
        if is_correct {
            correct.push(update.clone());
        } else {
            incorrect.push(update.clone());
        }
    }

    SplitUpdates { correct, incorrect }
}

fn sum_medians(updates: &Vec<Vec<i32>>) -> i32 {
    updates
        .iter()
        .fold(0, |acc, update| acc + update[update.len() / 2])
}

fn sort_updates(
    updates: &Vec<Vec<i32>>,
    after_lookup: &HashMap<i32, HashSet<i32>>,
) -> Vec<Vec<i32>> {
    updates
        .iter()
        .map(|u| {
            let mut update: Vec<i32> = u.to_vec().clone();
            update.sort_by(|a, b| {
                if after_lookup.get(a).unwrap_or(&HashSet::new()).contains(b) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            update
        })
        .collect()
}

fn main() {
    let test_data = parse_input_file("05.test.dat");
    let updates = split_updates(&test_data);
    assert!(updates.correct.len() == 3);
    assert!(updates.incorrect.len() == 3);
    assert!(sum_medians(&updates.correct) == 143);
    assert!(sum_medians(&sort_updates(&updates.incorrect, &test_data.after_lookup)) == 123);

    let data = parse_input_file("05.dat");
    let updates = split_updates(&data);
    println!("First answer: {}", sum_medians(&updates.correct));
    println!(
        "Second answer: {}",
        sum_medians(&sort_updates(&updates.incorrect, &data.after_lookup))
    );
}
