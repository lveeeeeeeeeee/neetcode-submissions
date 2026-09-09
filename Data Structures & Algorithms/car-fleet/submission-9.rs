use std::cmp;

#[derive(Debug)]
struct Pair {
    position: i32,
    velocity: i32,
}

impl Pair {
    fn new(position: i32, velocity: i32) -> Pair {
        Pair {
            position,
            velocity,
        }
    }

    fn time_arrive(&self, target: i32) -> f64 {
        (target - self.position) as f64 / self.velocity as f64
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        other.position.cmp(&self.position)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.position.eq(&other.position) && self.velocity.eq(&other.velocity)
    }
}

impl Eq for Pair {}

impl Solution {

    pub fn car_fleet(target: i32, position: Vec<i32>, velocity: Vec<i32>) -> i32 {
        // car catches up only if the front car is slower
        // the slowest car thats the furthest from the start 
        // will lead the first car fleet
        // store this car, if some next car doesn't reach it
        // within time = (x2-x1//v1-v2) + 1 such that 
        // x2+v2*t <= target - this car will lead the next
        // car fleet.
        if position.len() == 1 {
            return 1;
        }

        let mut sorted: Vec<Pair> = vec![];
        for i in 0..position.len() {
            sorted.push(Pair::new(position[i], velocity[i]));
        }
        sorted.sort();
        let sorted = sorted;
        let mut stack: Vec<f64> = vec![];
        for elem in &sorted {
            stack.push(elem.time_arrive(target));
            let len = stack.len();
            if len >= 2 {
                if stack[len-1] <= stack[len-2] {
                    stack.pop();
                }
            }
        }

        i32::try_from(stack.len()).unwrap()
    }
}
