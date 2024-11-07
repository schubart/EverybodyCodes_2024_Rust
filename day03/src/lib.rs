use std::collections::HashSet;

type Point = (isize, isize);
type Layer = HashSet<Point>;
type Direction = (isize, isize);

/// ```
/// let cardinal = [(-1, 0), (0, -1), (1, 0), (0, 1)];
/// let diagonal = [(-1, 0), (0, -1), (1, 0), (0, 1), (-1, -1), (1, -1), (1, 1), (-1, 1)];
///
/// assert_eq!(   120, day03::solve(include_str!("input1.txt"), &cardinal));
/// assert_eq!( 2_712, day03::solve(include_str!("input2.txt"), &cardinal));
/// assert_eq!(10_336, day03::solve(include_str!("input3.txt"), &diagonal));
/// ```
pub fn solve(input: &str, directions: &[Direction]) -> usize {
    let mut layer = Layer::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                layer.insert((x as isize, y as isize));
            }
        }
    }

    let mut result = 0;

    while !layer.is_empty() {
        result += layer.len();

        let has_all_neighbours = |(x, y): &Point| {
            directions
                .iter()
                .all(|(dx, dy)| layer.contains(&(x + dx, y + dy)))
        };

        layer = layer.iter().copied().filter(has_all_neighbours).collect();
    }

    result
}
