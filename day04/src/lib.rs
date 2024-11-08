type Nail = u32;
type TargetFn = fn(&mut [Nail]) -> Nail;

/// ```
/// assert_eq!(         80, day04::solve(include_str!("input1.txt"), day04::min));
/// assert_eq!(    824_608, day04::solve(include_str!("input2.txt"), day04::min));
/// assert_eq!(122_004_276, day04::solve(include_str!("input3.txt"), day04::median));
/// ```
pub fn solve(input: &str, target_fn: TargetFn) -> Nail {
    let mut nails: Vec<Nail> = input.lines().map(|s| s.parse().unwrap()).collect();

    let target = target_fn(&mut nails);

    nails.iter().map(|nail| nail.abs_diff(target)).sum()
}

pub fn min(nails: &mut [Nail]) -> Nail {
    *nails.iter().min().unwrap()
}

pub fn median(nails: &mut [Nail]) -> Nail {
    nails.sort_unstable();
    nails[nails.len() / 2]
}
