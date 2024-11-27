use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;

/// ```
/// assert_eq!(188, day15::solve(include_str!("input1.txt")));
/// ```
pub fn solve(input: &str) -> usize {
    let mut tiles = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '#' {
                tiles.insert((x as isize, y as isize), c);
            }
        }
    }

    let &start = tiles.keys().find(|&p| p.1 == 0).unwrap();

    let mut queue = VecDeque::from([start]);
    let mut distances = HashMap::from([(start, 0)]);

    while let Some(point) = queue.pop_front() {
        let distance = distances[&point];

        for direction in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next = (point.0 + direction.0, point.1 + direction.1);

            if tiles.contains_key(&next) && !distances.contains_key(&next) {
                distances.insert(next, distance + 1);
                queue.push_back(next);
            }
        }
    }

    tiles
        .iter()
        .filter_map(|(p, &c)| (c == 'H').then_some(distances[p]))
        .min()
        .unwrap()
        * 2
}

/// ```
/// assert_eq!(520, day15::solve2(include_str!("input2.txt")));
/// // assert_eq!(0, day15::solve2(include_str!("input3.txt")));
/// ```
pub fn solve2(input: &str) -> usize {
    let mut tiles = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '#' && c != '~' {
                tiles.insert((x as isize, y as isize), c);
            }
        }
    }

    let &start = tiles.keys().find(|&p| p.1 == 0).unwrap();

    let interesting: Vec<_> = tiles
        .iter()
        .filter_map(|(&p, &c)| (p == start || c != '.').then_some(p))
        .collect();
    // from pair (from, to) -> distance
    let mut matrix = HashMap::new();

    for &from in &interesting {
        let mut queue = VecDeque::from([from]);
        let mut distances = HashMap::from([(from, 0)]);

        while let Some(point) = queue.pop_front() {
            let distance = distances[&point];

            for direction in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let next = (point.0 + direction.0, point.1 + direction.1);

                if tiles.contains_key(&next) && !distances.contains_key(&next) {
                    distances.insert(next, distance + 1);
                    queue.push_back(next);
                }
            }
        }

        for &to in &interesting {
            let distance = distances[&to];
            matrix.insert((from, to), distance);
            matrix.insert((to, from), distance);
        }
    }

    let mut herbs = HashMap::<char, Vec<_>>::new();

    for (&p, &c) in tiles.iter() {
        if c != '.' {
            herbs.entry(c).or_default().push(p);
        }
    }

    let mut min = usize::MAX;
    for permutation in ['A', 'B', 'C', 'D', 'E'].iter().permutations(5) {
        for &a in &herbs[&permutation[0]] {
            let path = matrix[&(start, a)];
            if path >= min {
                continue;
            }

            for &b in &herbs[&permutation[1]] {
                let path = path + matrix[&(a, b)];
                if path >= min {
                    continue;
                }

                for &c in &herbs[&permutation[2]] {
                    let path = path + matrix[&(b, c)];
                    if path >= min {
                        continue;
                    }

                    for &d in &herbs[&permutation[3]] {
                        let path = path + matrix[&(c, d)];
                        if path >= min {
                            continue;
                        }

                        for &e in &herbs[&permutation[4]] {
                            let path = path + matrix[&(d, e)];
                            let path = path + matrix[&(e, start)];
                            min = min.min(path);
                        }
                    }
                }
            }
        }
    }

    min
}
