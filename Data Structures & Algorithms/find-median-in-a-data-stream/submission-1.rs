// insertion sort (but addition of an element is O(n)) (but median is O(1))
struct MedianFinder {
    collect: Vec<i32>
}

impl MedianFinder {
    pub fn new() -> Self {
        MedianFinder {
            collect: vec![0;0]
        }
    }

    pub fn add_num(&mut self, num: i32) {
        let mut reorder = Vec::with_capacity(self.collect.len());
        if self.collect.is_empty() {
            self.collect.push(num);
            return
        }
        while let Some(num_top) = self.collect.last() {
            if num < *num_top {
                reorder.push(self.collect.pop().unwrap());
            } else {
                break;
            }
        }
        self.collect.push(num);
        while let Some(r) = reorder.pop() {
            self.collect.push(r);
        }
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
