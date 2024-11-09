use std::collections::HashMap;
use std::collections::HashSet;

struct State {
    columns: Vec<Vec<usize>>,
    next_column: usize,
}

/// ```
/// assert_eq!(2_423, day05::solve(include_str!("input1.txt")));
/// ```
pub fn solve(input: &str) -> usize {
    let mut state = parse(input);

    for _ in 0..10 {
        state = progress(state);
    }

    shout(&state)
}

/// ```
/// assert_eq!(14319424693210, day05::solve2(include_str!("input2.txt")));
/// ```
pub fn solve2(input: &str) -> usize {
    let mut state = parse(input);
    let mut counts = HashMap::new();

    for round in 1.. {
        state = progress(state);

        let result = shout(&state);
        let count = *counts.entry(result).and_modify(|x| *x += 1).or_insert(1);

        if count == 2024 {
            return round * result;
        }
    }

    unreachable!()
}

/// ```
/// assert_eq!(9715100210001002, day05::solve3(include_str!("input3.txt")));
/// ```
pub fn solve3(input: &str) -> usize {
    let mut state = parse(input);

    let mut seen = HashSet::new();
    let mut max = 0;

    while seen.insert(state.columns.clone()) {
        state = progress(state);

        max = std::cmp::max(max, shout(&state));
    }

    max
}

fn parse(input: &str) -> State {
    let mut columns = Vec::<Vec<usize>>::new();
    for line in input.lines() {
        for (column, c) in line.split_whitespace().enumerate() {
            if columns.len() == column {
                columns.push(Vec::new());
            }
            columns[column].push(c.parse().unwrap());
        }
    }

    State {
        columns,
        next_column: 0,
    }
}

fn progress(mut state: State) -> State {
    let column = state.next_column % state.columns.len();
    state.next_column += 1;
    let player = state.columns[column].remove(0);
    let target = (column + 1) % state.columns.len();
    let target_column = &mut state.columns[target];

    let steps = 1 + (player - 1) % (2 * target_column.len());
    assert!(steps <= target_column.len() * 2);

    if steps <= target_column.len() {
        target_column.insert(steps - 1, player);
    } else {
        target_column.insert(1 + 2 * target_column.len() - steps, player);
    }

    state
}

fn shout(state: &State) -> usize {
    state.columns
        .iter()
        .map(|column| column[0])
        .map(|number| number.to_string())
        .collect::<String>()
        .parse()
        .unwrap()
}
