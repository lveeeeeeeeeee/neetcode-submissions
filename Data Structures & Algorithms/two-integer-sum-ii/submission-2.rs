impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l: usize = 0;
        let mut r: usize = numbers.len() - 1;
        while r > l {
            if target < numbers[r] + numbers[l] {
                r -= 1;
            }
            else if target > numbers[r] + numbers[l] {
                l += 1;
            }
            else {
                return Vec::from([1+l as i32, 1+r as i32]);  
            }
        }
        Vec::from([1+l as i32, 1+r as i32])
    }
}
