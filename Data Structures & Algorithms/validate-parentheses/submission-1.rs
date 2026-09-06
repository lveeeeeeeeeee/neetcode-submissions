impl Solution {
    pub fn to_push(sym: char) -> Option<char> {
        match sym {
            '{' => return Some('}'),
            '[' => return Some(']'),
            '(' => return Some(')'),
            _ => return None
        }
    }

    pub fn is_valid(s: String) -> bool {
        let elem: Option<char>;
        let mut stack: Vec<char> = Vec::new();
        for elem in s.chars() {
            match Self::to_push(elem) {
                Some(open_bracket) => stack.push(open_bracket),
                None => {
                    let closed: Option<char> = stack.pop();
                    match closed {
                        Some(sym) => {
                            if sym != elem {
                                return false;
                            }
                        },
                        None => {
                            return false;
                        }
                    }
                }
            }
        }
        if stack.len() != 0 {
            return false;
        }
        true
    }
}
