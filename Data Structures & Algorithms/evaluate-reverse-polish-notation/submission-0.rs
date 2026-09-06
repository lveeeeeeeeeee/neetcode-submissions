impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];
        for elem in tokens {
            match elem.parse::<i32>() {
                Ok(num) => { 
                    stack.push(num); 
                    continue;
                },
                Err(..) => { }
            }
            let second: i32 = stack.pop().unwrap();
            let first: i32 = stack.pop().unwrap();
            match elem.as_str() {
                "+" => { stack.push(first + second); },
                "-" => { stack.push(first - second); },
                "*" => { stack.push(first * second); },
                "/" => { stack.push(first / second); },
                _ => {}
            }
        }
        stack.pop().unwrap()
    }
}
