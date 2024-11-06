type Score = i32;

/// ```
/// assert_eq!( 1_328, day01::score(include_str!("input1.txt"), 1));
/// assert_eq!( 5_626, day01::score(include_str!("input2.txt"), 2));
/// assert_eq!(27_565, day01::score(include_str!("input3.txt"), 3));
/// ```
pub fn score(input: &str, group_size: usize) -> Score {
    input
        .trim()
        .as_bytes()
        .chunks(group_size)
        .map(score_group)
        .sum()
}

fn score_group(group: &[u8]) -> Score {
    let score: Score = group.iter().copied().filter_map(score_monster).sum();
    let count = group.iter().copied().filter_map(score_monster).count() as Score;
    let boost = (count - 1) * count; // n - 1 per monster for a group of n.

    score + boost
}

fn score_monster(monster: u8) -> Option<Score> {
    match monster {
        b'x' => None,
        b'A' => Some(0),
        b'B' => Some(1),
        b'C' => Some(3),
        b'D' => Some(5),
        _ => unimplemented!("Monster {}", monster),
    }
}
