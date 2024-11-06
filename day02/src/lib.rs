#![allow(clippy::missing_panics_doc)]

use std::collections::HashMap;
use std::collections::HashSet;

type Words = Vec<String>;
type Inscription = HashMap<(isize, isize), char>;

/// ```
/// assert_eq!(33, day02::part1());
/// ```
pub fn part1() -> usize {
    let mut lines = include_str!("input1.txt").lines();
    let words: Vec<_> = lines
        .next()
        .unwrap()
        .strip_prefix("WORDS:")
        .unwrap()
        .split(',')
        .collect();
    let inscription = lines.nth(1).unwrap();

    words
        .iter()
        .map(|word| inscription.matches(word).count())
        .sum()
}

/// ```
/// assert_eq!(5_280, day02::part2());
/// ```
pub fn part2() -> usize {
    let mut lines = include_str!("input2.txt").lines();
    let words: Vec<_> = lines
        .next()
        .unwrap()
        .strip_prefix("WORDS:")
        .unwrap()
        .split(',')
        .collect();

    lines.next();

    lines
        .map(|inscription| {
            let mut rune_indices = HashSet::new();

            for word in &words {
                for index in 0..inscription.len() {
                    if inscription[index..].starts_with(word) {
                        for offset in 0..word.len() {
                            rune_indices.insert(index + offset);
                        }
                    }
                }
            }
            for word in &words {
                for index in 0..inscription.len() {
                    if inscription[index..].starts_with(&word.chars().rev().collect::<String>()) {
                        for offset in 0..word.len() {
                            rune_indices.insert(index + offset);
                        }
                    }
                }
            }

            rune_indices.len()
        })
        .sum()
}

/// ```
/// assert_eq!(11_667, day02::part3());
/// ```
pub fn part3() -> usize {
    let (words, inscription) = parse(include_str!("input3.txt"));
    let width = inscription.keys().map(|key| key.0).max().unwrap() + 1;

    let mut indices = HashSet::new();

    for word in words {
        for (x, y) in inscription.keys() {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let keys = || {
                    (0..word.len()).map(|offset| {
                        (
                            (x + (offset as isize) * dx).rem_euclid(width),
                            (y + (offset as isize) * dy),
                        )
                    })
                };

                if keys()
                    .zip(word.chars())
                    .all(|(key, c)| inscription.get(&key) == Some(&c))
                {
                    indices.extend(keys());
                }
            }
        }
    }

    indices.len()
}

fn parse(input: &str) -> (Words, Inscription) {
    let mut lines = input.lines();
    let words: Vec<_> = lines
        .next()
        .unwrap()
        .strip_prefix("WORDS:")
        .unwrap()
        .split(',')
        .map(str::to_string)
        .collect();

    lines.next();

    let mut inscription = Inscription::new();

    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            inscription.insert((x as isize, y as isize), c);
        }
    }

    (words, inscription)
}
