use std::collections::HashMap;

struct TimeMap {
    __map: HashMap<String, Vec<(String, i32)>>
}

impl TimeMap {
    fn search_insert(self: &Self, ts: i32, arr: &Vec<(String, i32)>, mut l: usize, mut r: usize) -> Option<usize> {
        if arr.len() == 0 {
            return None;
        }
        while l < r {
            let m: usize = (r + l) / 2;
            if ts < arr[m].1 {
                r = m - 1;
            } else {
                l = m + 1;
            }
        }
        if l == 0 && ts < arr[0].1 {
            // since every timestamp is increasing
            // this shouldnt be reached
            return None;
        }
        Some(l)
    }

    fn search(self: &Self, ts: i32, arr: &Vec<(String, i32)>, mut l: usize, mut r: usize) -> Option<usize> {
        if arr.len() == 0 {
            return None;
        }
        while l < r {
            let m: usize = (r + l + 1) / 2;
            if ts < arr[m].1 {
                r = m - 1;
            } else {
                l = m;
            }
        }
        if l == 0 && ts < arr[0].1 {
            // since every timestamp is increasing
            // this shouldnt be reached
            return None;
        }
        // println!("{}", l);
        Some(l)
    }

    fn new() -> Self {
        TimeMap { __map: HashMap::new() }
    }

    fn set(self: &mut Self, key: String, value: String, timestamp: i32) {
        if self.__map.contains_key(&key) {
            let mut v = std::mem::take(self.__map.get_mut(&key).unwrap());
            if let Some(ind) = self.search_insert(timestamp, &v, 0, v.len()) {
                v.insert(ind, (value, timestamp));
            }
            self.__map.insert(key, std::mem::take(&mut v));
        } else {
            let mut nv = Vec::new();
            nv.push((value, timestamp));
            self.__map.insert(key, std::mem::take(&mut nv));
        }
    }

    fn get(self: &Self, key: String, timestamp: i32) -> String {
        if self.__map.contains_key(&key) {
            let v = self.__map.get(&key).unwrap();
            if !v.is_empty() && let Some(ind) = self.search(timestamp, v, 0, v.len() - 1) {
                if ind > v.len() {
                    return v[v.len() - 1].0.clone();
                } else {
                    return v[ind].0.clone();
                }
            }
        }
        String::default()
    }
}
