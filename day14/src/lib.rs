use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

type IsLeaf = bool;
type Point = (isize, isize, isize);
pub type Segments = HashMap<Point, IsLeaf>;

/// ```
/// assert_eq!(144,                day14::grow(include_str!("input1.txt")).keys().map(|p| p.0).max().unwrap());
/// assert_eq!(4848,               day14::grow(include_str!("input2.txt")).len());
/// assert_eq!(1326, day14::part3(&day14::grow(include_str!("input3.txt"))));
/// ```
pub fn grow(input: &str) -> Segments {
    let mut result = Segments::new();

    for line in input.lines() {
        let mut current = (0, 0, 0);

        for (direction, steps) in line
            .split(',')
            .map(|step| step.split_at(1))
            .map(|(direction, steps)| (direction, steps.parse().unwrap()))
        {
            for _ in 0..steps {
                current = neighbour(current, direction);
                result.insert(current, false);
            }
        }

        result.insert(current, true);
    }

    result
}

pub fn part3(segments: &Segments) -> usize {
    // Map each trunk segment to the sum of distances to leaves.
    let mut leaf_distances = HashMap::<Point, usize>::new();

    for leaf in segments
        .iter()
        .filter_map(|(&p, is_leaf)| is_leaf.then_some(p))
    {
        let mut distances = HashMap::<_, _>::from([(leaf, 0)]);
        let mut queue = BinaryHeap::from([(Reverse(0), leaf)]);

        while let Some((_, current)) = queue.pop() {
            // TODO: Don't need full Dijkstra here.
            let current_distance = distances[&current];

            if current.1 == 0 && current.2 == 0 {
                *leaf_distances.entry(current).or_default() += current_distance;
            }

            for direction in ["U", "D", "R", "L", "F", "B"] {
                let neighbour = neighbour(current, direction);

                if segments.contains_key(&neighbour) {
                    let new_distance = current_distance + 1;

                    if new_distance < *distances.get(&neighbour).unwrap_or(&usize::MAX) {
                        distances.insert(neighbour, new_distance);
                        queue.push((Reverse(new_distance), neighbour));
                    }
                }
            }
        }
    }

    *leaf_distances.values().min().unwrap()
}

fn neighbour(point: Point, direction: &str) -> Point {
    let mut result = point;

    match direction {
        "U" => result.0 += 1,
        "D" => result.0 -= 1,
        "R" => result.1 += 1,
        "L" => result.1 -= 1,
        "F" => result.2 += 1,
        "B" => result.2 -= 1,
        _ => unreachable!(),
    }

    result
}
