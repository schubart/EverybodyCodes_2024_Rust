use std::collections::HashMap;
use std::collections::HashSet;

/// ```
/// assert_eq!("FVWBQSRHPKGTDLNC", day10::part1());
/// ```
pub fn part1() -> String {
    let mut runes = HashMap::new();
    let mut gaps = Vec::new();
    for (y, line) in include_str!("input1.txt").lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let _ = match c {
                '.' => gaps.push((x, y)),
                '*' => (),
                _ => {
                    runes.insert((x, y), c);
                }
            };
        }
    }

    let mut result = String::new();
    for (x, y) in gaps {
        let row = runes
            .iter()
            .filter_map(|((rune_x, _rune_y), rune)| (*rune_x == x).then_some(rune))
            .collect::<HashSet<_>>();
        let column = runes
            .iter()
            .filter_map(|((_rune_x, rune_y), rune)| (*rune_y == y).then_some(rune))
            .collect::<HashSet<_>>();

        let mut intersection = row.intersection(&column);
        let rune = **intersection.next().unwrap();
        assert!(intersection.next().is_none());

        runes.insert((x, y), rune);
        result.push(rune)
    }

    result
}

/// ```
/// assert_eq!(194753, day10::part2());
/// ```
pub fn part2() -> usize {
    let mut chars: Vec<Vec<char>> = include_str!("input2.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut sum  = 0;
    
    for grid_y in 0..chars.len().div_ceil(9) {
        for grid_x in 0..chars[0].len().div_ceil(9) {
            let mut result = String::new();
            for dy in 2..6 {
                for dx in 2..6 {
                    assert_eq!('.', chars[grid_y * 9 + dy][grid_x * 9 + dx]);
                    let row = (0..8)
                        .filter_map(|sy| {
                            let c = chars[grid_y * 9 + sy][grid_x * 9 + dx];
                            (c != '.').then_some(c)
                        })
                        .collect::<HashSet<_>>();
                    let column = (0..8)
                        .filter_map(|sx| {
                            let c = chars[grid_y * 9 + dy][grid_x * 9 + sx];
                            (c != '.').then_some(c)
                        })
                        .collect::<HashSet<_>>();

                    let mut intersection = row.intersection(&column);
                    let rune = *intersection.next().unwrap();
                    assert!(intersection.next().is_none());

                    chars[grid_y * 9 + dy][grid_x * 9 + dx] = rune;
                    result.push(rune)
                }
            }

            sum += power(&result);
        }
    }

    sum
}

fn power(runes: &str) -> usize {
    runes
        .chars()
        .enumerate()
        .map(|(index, c)| (index + 1) * (1 + c as usize - b'A' as usize))
        .sum()
}
