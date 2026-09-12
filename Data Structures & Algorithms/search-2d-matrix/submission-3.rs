impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut lr: usize = 0;
        let mut rr: usize = matrix.len()-1;
        let mut m: usize = matrix[0].len() / 2;
        let mut mr: usize = lr + (rr - lr / 2);
        
        // looking in which row the element might be
        while lr <= rr {
            mr = lr + ((rr - lr) / 2);
            if target > matrix[mr][matrix[mr].len()-1] {
                lr = mr + 1;
            } else if target < matrix[mr][0] {
                if mr == 0 {
                    return false;
                }
                rr = mr - 1;
            } else {
                break;
            }
        }
        
        if lr > rr { return false; }
        
        // binary search of the element itself
        let mut l: i32 = 0;
        let mut r: i32 = matrix[0].len() as i32 - 1;
        while l <= r {
            m = (l + ((r - l) / 2)) as usize;
            if matrix[mr][m] == target {
                return true;
            } else if target > matrix[mr][m] {
                l = m as i32 + 1;
            } else if target < matrix[mr][m] {
                r = m as i32 - 1;
            }
        }
        matrix[mr][m] == target
    }
}