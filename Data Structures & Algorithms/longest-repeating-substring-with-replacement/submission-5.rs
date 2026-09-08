use std::collections::HashSet;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let arr: Vec<char> = s.chars().collect();
        let mut max: i32 = 0;
        let mut max_w: i32 = 0;
        
        let mut map: HashMap<char, i32> = HashMap::new();
        let mut l: usize = 0;

        for r in 0..arr.len() {
            let f = map.entry(arr[r]).and_modify(|amt| *amt += 1).or_insert(1);
            max_w = max_w.max(*f);

            while (r - l + 1) as i32 - max_w > k {
                map.entry(arr[l]).and_modify(|amt| *amt -= 1);
                l += 1;
            }
            max = max.max((r - l + 1) as i32);
        }
        max
    }
}
