use std::collections::HashMap;

impl Solution {
    // dfs lmao
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if x == 1f64 || n == 1 {
            return x;
        } else if n == -1 {
            return 1f64/x;
        } else if n == 0 {
            return 1f64;
        }
        let mut dfs: Vec<i32> = Vec::new();
        let mut map: HashMap<i32, f64> = HashMap::new();
        let bm: &mut HashMap<i32, f64> = &mut map;
        bm.insert(1, x);
        bm.insert(-1, 1f64/x);
        bm.insert(0, 1f64);
        dfs.push(n);
        while let Some(&elem) = dfs.last() {
            if elem != 1 && elem != -1 && elem != 0 {
                if !bm.contains_key(&(elem / 2)) {
                    // println!("going elem / 2: elem is {}, elem / 2 is {}", elem, elem / 2);
                    dfs.push(elem / 2);
                    continue;
                } 
                if !bm.contains_key(&(elem - (elem / 2))) {
                    // println!("going elem / 2: elem is {}, elem - (elem / 2) is {}", elem, elem - (elem / 2));
                    dfs.push(elem - (elem / 2));
                    continue;
                }   
                // we just checked that both elem / 2 
                // and elem - (elem / 2) are in the map.
                // store the power in the map
                // println!("yo getting an x^{} and x^{} results", elem / 2, elem - (elem / 2));
                let m1 = *bm.get(&(elem / 2)).unwrap();
                let m2 = *bm.get(&(elem - (elem / 2))).unwrap();
                bm.insert(elem, m1 * m2);
                // println!("just inserted {} to key {}", m1 * m2, elem);
                dfs.pop();
            }
        }
        *bm.get(&n).unwrap()
    }
}