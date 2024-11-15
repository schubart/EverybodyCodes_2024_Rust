use std::cmp::min;

/// ```
/// assert_eq!(11815, day09::parts1_2(include_str!("input1.txt"), &[1, 3, 5, 10]));
/// assert_eq!(5251,  day09::parts1_2(include_str!("input2.txt"), &[1, 3, 5, 10, 15, 16, 20, 24, 25, 30]));
/// ```
pub fn parts1_2(input: &str, stamps: &[usize]) -> usize {
    let (targets, max_target) = parse(input);
    let solutions = solve(max_target, stamps);

    targets.iter().map(|&target| solutions[target]).sum()
}

/// ```
/// assert_eq!(144802, day09::part3());
/// ```
pub fn part3() -> usize {
    let (targets, max_target) = parse(include_str!("input3.txt"));
    let stamps = [
        1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101,
    ];
    let solutions = solve(max_target, &stamps);

    targets
        .iter()
        .map(|&target| target.div_ceil(2))
        .map(|mid| {
            (0..=50)
                .map(|offset| solutions[mid + offset] + solutions[mid - offset])
                .min()
                .unwrap()
        })
        .sum()
}

fn parse(input: &str) -> (Vec<usize>, usize) {
    let targets: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();
    let &max_target = targets.iter().max().unwrap();

    (targets, max_target)
}

// Builds a vec where element `n` is the minimum number of beetles required for brightness `n`.
fn solve(max_target: usize, stamps: &[usize]) -> Vec<usize> {
    // Initial best solutions: `n` beetles for brightness `n`, using only "1" stamp.
    assert!(stamps.contains(&1));
    let mut solutions: Vec<usize> = (0..=max_target).collect();

    // Use every kind of stamp to try and improve solutions.
    for &stamp in stamps {
        for target in stamp..=max_target {
            solutions[target] = min(
                solutions[target],             // Current best solution.
                solutions[target - stamp] + 1, // Best solution w/o stamp, plus 1 stamp.
            );
        }
    }

    solutions
}
