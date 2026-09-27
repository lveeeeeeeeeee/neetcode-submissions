impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<(i32, usize)> = Vec::new();
        let mut res: usize = 0;
        for i in 0..heights.len() {
            let mut start = i;
            while let Some(&(h, ind)) = stack.last() {
                if h > heights[i] {
                    stack.pop();
                    res = res.max(h as usize * (i - ind));
                    start = ind;
                }
                else { 
                    break; 
                }
            }
            stack.push((heights[i], start));
        }

        while let Some((h, ind)) = stack.pop() {
            res = res.max(h as usize * (heights.len() - ind));
        }
        res as i32
    }
}
