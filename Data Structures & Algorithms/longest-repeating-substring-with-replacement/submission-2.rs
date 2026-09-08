use std::collections::HashSet;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let arr: Vec<char> = s.chars().collect();
        let u: HashSet<char> = HashSet::from_iter(s.chars());
        let mut max: i32 = 0;
        
        for uc in u {
            let mut map: HashMap<char, i32> = HashMap::new();
            let mut l: usize = 0;
            let mut r: usize = 0;
            
            // repititions
            let mut reps: i32 = 0;
            
            loop {
                if r >= arr.len() {
                    break;
                }
                // we grow the window until reps <= k
                // if reps > k we need to shrink the window until it isnt true
                // reps = r - l - map.get(arr[l])
                while r < arr.len() && reps <= k {
                    map.entry(arr[r]).and_modify(|amt| *amt += 1).or_insert(1);
                    if uc != arr[r] {
                        reps += 1;
                    }
                    max = if reps <= k { max.max(1 + (r - l) as i32) } else { max };
                    let deb_l = arr[l];
                    let deb_r = arr[r];
                    // println!("{deb_l:?}, now: {deb_r:?}; (r-l): {}, reps {reps:?}; max {max:?}", 1 + (r - l) as i32);
                    r += 1;
                }
                while l < r && reps > k {
                    map.entry(arr[l]).and_modify(|amt| *amt -= 1);
                    l += 1;
                    reps = ((r - l) as i32) - *map.entry(uc).or_insert(0);
                    // println!("shrink: (r-l) {}, reps {}", (r - l) as i32, reps);
                }
            }
        }
        max
    }
}
