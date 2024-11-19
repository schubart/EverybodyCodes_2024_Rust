use std::collections::HashMap;

type Termite = &'static str;
type Transitions = HashMap<Termite, Vec<Termite>>;
type Counts = HashMap<Termite, usize>;

/// ```
/// assert_eq!(           32,               day11::simulate(include_str!("input1.txt"),  4)["A"]);
/// assert_eq!(       230692,               day11::simulate(include_str!("input2.txt"), 10)["Z"]);
/// assert_eq!(1276473674450, day11::part3(&day11::simulate(include_str!("input3.txt"), 20)));
/// ```
pub fn simulate(input: &'static str, days: usize) -> Counts {
    let transitions: Transitions = input
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(from, to)| (from, to.split(',').collect()))
        .collect();

    (0..days).fold(Counts::new(), |counts, _| {
        transitions
            .iter()
            .map(|(&from, to)| (from, to.iter().map(|t| counts.get(t).unwrap_or(&1)).sum()))
            .collect()
    })
}

pub fn part3(counts: &Counts) -> usize {
    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();

    max - min
}
