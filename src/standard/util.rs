pub fn collision_check_rect_point(
    rect_pos: (f32, f32),
    rect_size: (f32, f32),
    point: (f32, f32),
) -> bool {
    let (x, y) = point;
    let (left, top) = rect_pos;
    let (right, bottom) = (left + rect_size.0, top + rect_size.1);

    if left <= x && x <= right && top <= y && y <= bottom {
        true
    } else {
        false
    }
}
