impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // fibonacci numbers lmao
        let mut list: Vec<i32> = vec![0; n as usize];
        for i in 0..list.len() {
            if i == 0 {
                list[i] = 1;
            }
            else if i == 1 {
                list[i] = 2;
            }
            else {
                list[i] = list[i-1] + list[i-2];
            }
        }
        list[n as usize - 1]
    }
}
