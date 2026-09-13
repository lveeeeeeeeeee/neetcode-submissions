// wdym O(1) to find a min element


struct MinStack {
    min: Option<i64>,
    stack: Vec<i64>
}

impl MinStack {
    pub fn new() -> Self {
        let stack: Vec<i64> = Vec::new();
        MinStack { min: None, stack }
    }

    pub fn push(&mut self, val: i32) {
        let val = val as i64;
        if let Some(mut cur_min) = self.min {
            if val < cur_min {
                self.min = Some(val);
            }
            self.stack.push(val - cur_min);
        } else {
            self.min = Some(val);
            self.stack.push(0);
        }
    }

    pub fn pop(&mut self) {
        if !self.stack.is_empty() {
            let popped = self.stack.pop().unwrap();
            if popped < 0 {
                self.min = Some(self.min.unwrap() - popped);
            } else if popped == 0 && self.stack.len() == 0 {
                self.min = None;
            }
        }
    }

    pub fn top(&self) -> i64 {
        if let (Some(peek), Some(cur_min)) = (self.stack.last(), self.min) {
            if *peek <= 0 { return cur_min; }
            return *peek + cur_min;
        }
        // genuinely dont know what do you want from me 
        // if you dont allow me to return an option
        i64::MIN 
    }

    pub fn get_min(&self) -> i64 {
        if let Some(cur_min) = self.min {
            return cur_min;
        }
        i64::MIN
    }
}
