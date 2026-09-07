use std::iter::zip;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<(usize, i32)> = vec![];

        // 10^9 fits into 32 bits
        let mut max: i32 = 0;

        for (i, &elem) in heights.iter().enumerate() {
            let mut start = i;
            while let Some(&(ih, h)) = stack.last() {
                if h > elem {
                    stack.pop();
                    let thing = h * (i - ih) as i32;
                    max = max.max(thing);
                    start = ih;
                } else {
                    break;
                }
            }
            stack.push((start, elem));
        }
        let n: usize = heights.len();
        for &(index, height) in &stack {
            max = max.max(height * (n - index) as i32);
        }
        max
    }
}
