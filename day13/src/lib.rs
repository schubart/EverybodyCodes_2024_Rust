use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

/// ```
/// assert_eq!(145, day13::solve(include_str!("input1.txt")));
/// assert_eq!(612, day13::solve(include_str!("input2.txt")));
/// assert_eq!(543, day13::solve(include_str!("input3.txt")));
/// ```
pub fn solve(input: &str) -> usize {
    let mut starts = Vec::new();
    let mut ends = Vec::new();
    let mut heights = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let point = (x as isize, y as isize);

            match c {
                ' ' | '#' => (),
                'S' => {
                    starts.push(point);
                    heights.insert(point, 0);
                }
                'E' => {
                    ends.push(point);
                    heights.insert(point, 0);
                }
                _ => {
                    heights.insert(point, c.to_digit(10).unwrap() as usize);
                }
            }
        }
    }

    let mut distances = HashMap::<_, _>::from_iter(starts.iter().map(|&start| (start, 0)));
    let mut queue = BinaryHeap::from_iter(starts.iter().map(|&start| (Reverse(0), start)));

    while let Some((_, current)) = queue.pop() {
        let current_height = heights[&current];
        let current_distance = distances[&current];

        for direction in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let neighbour = (current.0 + direction.0, current.1 + direction.1);

            if let Some(&neighbour_height) = heights.get(&neighbour) {
                let height_diff = current_height.abs_diff(neighbour_height);
                let height_diff = std::cmp::min(height_diff, 10 - height_diff);
                let new_distance = current_distance + height_diff + 1;

                // TODO: Can do this more nicely, without using MAX?
                if new_distance < *distances.get(&neighbour).unwrap_or(&usize::MAX) {
                    distances.insert(neighbour, new_distance);
                    queue.push((Reverse(new_distance), neighbour));
                }
            }
        }
    }

    ends.iter().map(|point| distances[point]).min().unwrap()
}
