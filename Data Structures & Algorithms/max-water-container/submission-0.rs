impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut l: usize = 0;
        let mut r: usize = heights.len() - 1;
        let mut res: usize = 0;
        while r > l {
            let w = r - l;
            let mov = heights[r].min(heights[l]);
            res = res.max(w * (mov as usize));
            if mov == heights[r] {
                r -= 1;
            } else {
                l += 1;
            }
        }
        return res as i32;
    }
}
