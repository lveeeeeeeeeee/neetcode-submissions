impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        if nums1.len() == 0 && nums2.len() == 0 {
            return 0f64;
        }
        if nums1.len() > nums2.len() {
            std::mem::swap(&mut nums1, &mut nums2);
        }
        let total = nums1.len() + nums2.len();
        let med = (total + 1) / 2;
        // the small array can be empty too
        let mut r: usize = nums1.len();
        let mut l: usize = 0usize;
        loop {
            let ssl: usize = (r + l) / 2;
            // we are finding the of length (^total / 2^) 
            // such that median is not contained in it.
            // always keep the sum of lengths. if we shrink one slice,
            // the other slice should be stretched by the same amount
            let bsl = med - ssl;
            // theres at least 1 element
            // if theres 1 element we are taking max from that as just arr[0]
            // but then min of that is strictly less than arr[max] so it is out of bounds
            // just say its neg infinity
            let max_sl = if ssl < nums1.len() { nums1[ssl] as f64 } else { f64::INFINITY };
            let min_sl = if ssl > 0 { nums1[ssl-1] as f64 } else { f64::NEG_INFINITY };
            let max_bl = if bsl < nums2.len() { nums2[bsl] as f64 } else { f64::INFINITY } ;
            let min_bl = if bsl > 0 { nums2[bsl-1] as f64 } else { f64::NEG_INFINITY };
            // cant shrink or stretch any slice further
            // achieved the goal of having a slice of satisfactory length
            if min_bl <= max_sl && min_sl <= max_bl {
                if total & 1 == 0 {
                    return (min_sl.max(min_bl) + max_sl.min(max_bl)) / 2f64;
                }
                return min_sl.max(min_bl) as f64;
            } else if min_sl > max_bl {
                // shrink min
                r = ssl - 1;
            } else {
                l = ssl + 1;
            }
        }
    }
}
