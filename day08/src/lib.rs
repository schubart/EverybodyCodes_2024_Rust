/// ```
/// assert_eq!(21,        day08::solve12(13,      |_| 1));
/// assert_eq!(7324641,   day08::solve12(4098816, |_| 1));
///
/// assert_eq!(27,        day08::solve12(50,       |thickness| (thickness * 3) % 5));
/// assert_eq!(106098210, day08::solve12(20240000, |thickness| (thickness * 998) % 1111));
/// ```
pub fn solve12(blocks: usize, next_thickness: fn(usize) -> usize) -> usize {
    let mut width = 1;
    let mut volume = 1;
    let mut thickness = 1;

    while volume < blocks {
        thickness = next_thickness(thickness);
        width += 2;
        volume += thickness * width;
    }

    width * (volume - blocks)
}
