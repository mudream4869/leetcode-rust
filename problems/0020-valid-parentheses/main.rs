use std::collections::VecDeque;
use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut map: HashMap<char, char> = HashMap::new();
        map.insert(']', '[');
        map.insert(')', '(');
        map.insert('}', '{');
        let mut stk: VecDeque<char> = VecDeque::new();
        for i in 0..s.len() {
            let c = s.chars().nth(i).unwrap();
            match map.get(&c) {
                None => {
                    stk.push_back(c);
                },
                Some(&cp) => {
                    if stk.back() == Some(&cp) {
                        stk.pop_back();
                    } else {
                        return false;
                    }
                },
            }
        }
        stk.len() == 0
    }
}
