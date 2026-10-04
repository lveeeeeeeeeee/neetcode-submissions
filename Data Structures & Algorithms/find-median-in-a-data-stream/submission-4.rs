// insertion sort (but addition of an element is O(n)) (but median is O(1))
struct MedianFinder {
    collect: VecDeque<i32>
}

impl MedianFinder {
    pub fn new() -> Self {
        MedianFinder {
            collect: VecDeque::new()
        }
    }

    pub fn add_num(&mut self, num: i32) {
        let mut collect = &mut self.collect;
        if collect.is_empty() {
            collect.push_back(num);
            return
        }
        let mut l = 0usize;
        let mut r = collect.len();
        while l < r {
            let m = l + ((r - l) / 2);
            if num > collect[m] {
                l = m + 1;
            } else {
                r = m;
            }
        }
        collect.insert(l, num);
    }

    pub fn find_median(&self) -> f64 {
        let collect = &self.collect;
        if collect.is_empty() {
            return 0f64
        } else {
            let med = (collect.len() / 2);
            if collect.len() & 1 == 0 {
                return (collect[med-1] + collect[med]) as f64 / 2 as f64;
            } else {
                return collect[med] as f64
            }
        }
    }
}
