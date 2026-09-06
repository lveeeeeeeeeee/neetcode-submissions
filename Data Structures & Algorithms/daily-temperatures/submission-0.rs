impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<usize> = vec![];
        let mut result: Vec<i32> = vec![0; temperatures.len()];
        for i in 0..temperatures.len() {
            if !stack.is_empty() {
                while let Some(&peek) = stack.last() {
                    if temperatures[i] > temperatures[peek] {
                        stack.pop();
                        result[peek] = i32::try_from(i - peek).unwrap();
                    }
                    else {
                        break;
                    }
                }
            }
            stack.push(i);
        }
        result
    }
}