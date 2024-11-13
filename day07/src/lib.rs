/// ```
/// assert_eq!("GHBJKFCED", day07::solve(include_str!("input1.txt")));
/// ```
pub fn solve(input: &str) -> String {
    let mut sums = input
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(plan, actions)| (plan, simulate("=", actions, 10)))
        .collect::<Vec<_>>();

    sums.sort_unstable_by_key(|(_plan, sum)| *sum);

    sums.iter().rev().map(|(plan, _sum)| *plan).collect()
}

/// ```
/// assert_eq!("IKJHDBCAG", day07::solve2(include_str!("input2.txt")));
/// ```
pub fn solve2(input: &str) -> String {
    let track = parse_track(include_str!("track.txt"));

    let mut sums = input
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(plan, actions)| (plan, simulate(&track, actions, 10)))
        .collect::<Vec<_>>();

    sums.sort_unstable_by_key(|(_plan, sum)| *sum);
    sums.iter().rev().map(|(plan, _sum)| *plan).collect()
}

/// ```
/// assert_eq!(6_942, day07::solve3());
/// ```
pub fn solve3() -> usize {
    let track = parse_track(include_str!("track2.txt"));
    let max = simulate(&track, "-,+,=,+,=,-,+,+,-,=,+", 2024);
    let mut count = 0;
    let mut actions = String::new();
    let upper = 3_usize.pow(11 as u32);
    dbg!(upper);
    for j in 0..upper {
        let mut i = j;
        actions.clear();
        let mut plus = 0;
        let mut minus = 0;
        let mut equal = 0;
        for _ in 0..11 {
            actions.push(match i % 3 {
                0 => {
                    plus += 1;
                    '+'
                }
                1 => {
                    minus += 1;
                    '-'
                }
                2 => {
                    equal += 1;
                    '='
                }
                _ => unreachable!(),
            });
            i /= 3;
            actions.push(',');
        }

        actions.pop();
        assert_eq!(11, plus + minus + equal);

        if plus != 5 || minus != 3 {
            continue;
        }

        let score = simulate(&track, &actions, 2024);
        if score > max {
            count += 1;
        }
    }

    count
}

fn parse_track(input: &str) -> String {
    let mut terrain = std::collections::HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != ' ' {
                terrain.insert((x as isize, y as isize), c);
            }
        }
    }

    let mut result = String::default();
    result.push(terrain[&(1, 0)]);

    let mut position = (1, 0);
    terrain.remove(&(0, 0));
    terrain.remove(&(1, 0));

    'outer: while !terrain.is_empty() {
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let candidate = (position.0 + dx, position.1 + dy);
            if let Some(c) = terrain.remove(&candidate) {
                position = candidate;
                result.push(c);
                continue 'outer;
            }
        }

        panic!();
    }

    result.push('=');
    result
}

fn simulate(track: &str, actions: &str, rounds: usize) -> usize {
    let mut actions = actions.split(',').cycle();

    let mut power: usize = 10;
    let mut sum = 0;
    for _ in 0..rounds {
        for terrain in track.chars() {
            let action = actions.next().unwrap();
            power = match (terrain, action) {
                ('+', _) => power + 1,
                ('-', _) => power - 1,
                ('=', "+") => power + 1,
                ('=', "-") => power.checked_sub(1).unwrap(),
                ('=', "=") => power,
                _ => unimplemented!("{terrain} {action}"),
            };

            sum += power;
        }
    }

    sum
}
