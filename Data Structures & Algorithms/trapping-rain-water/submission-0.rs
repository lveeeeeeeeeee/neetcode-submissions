impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        // 2 * 10^9 fits into i32
        let mut total: i32 = 0;
        let mut l: usize = 0;
        let mut r: usize = height.len() - 1;
    
        while r > l {
            let mut low = if height[l] < height[r] { height[l] } else { height[r] };
            let mi: &mut usize;
            let mx: &mut usize;
            if height[l] > height[r] { 
                mi = &mut r;
                mx = &mut l;
            } 
            else { 
                mi = &mut l;
                mx = &mut r;
            };
            
            let sign: bool = *mi < *mx;
            
            // println!("{sign:?}  mi {} - {}; mx {} - {}; total: {total:?}", *mi, height[*mi], *mx, height[*mx]);
            
            while height[*mi] <= height[*mx] && *mi != *mx {
                low = low.max(height[*mi]);
                let add = low - height[*mi];
                total += if low - height[*mi] > 0 { low - height[*mi] } else { 0 };
                // println!("{}, total: {total:?}, added {add:?}, low {low:?}", mi);
                *mi = if sign {*mi + 1} else {*mi - 1};
            }
        }
        total
    }
}
