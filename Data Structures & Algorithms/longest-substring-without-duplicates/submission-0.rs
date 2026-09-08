impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let arr: Vec<char> = s.chars().collect();
        let mut mask: u128 = 0;
        let mut l: usize = 0;
        let mut r: usize = 0;
        let mut max: usize = 0;
        loop {
            if r >= arr.len() {
                break;
            }
            let shift_amt: u128 = (arr[r] as u128) - 65u128;
            let shift: u128 = 1 << shift_amt;
            if mask & shift != 0 {
                while mask & shift != 0 {
                    let sl_amt: u128 = (arr[l] as u128) - 65u128;
                    let sl: u128 = 1 << sl_amt;
                    mask = mask ^ sl;
                    l += 1;
                }
            }
            else {
                mask = mask ^ shift;
                r += 1;
                max = max.max(r-l);
            }
        }
        i32::try_from(max).unwrap()
    }
}
