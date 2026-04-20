pub fn max_area(height: Vec<i32>) -> i32 {
    use std::cmp::{max, min};
    let mut result = 0;

    let mut left = 0;
    let mut right = height.len() - 1;

    while left < right {
        let area = (right - left) as i32 * min(height[left], height[right]);
        result = max(result, area);

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    result
}
