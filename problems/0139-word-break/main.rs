use std::collections::HashMap;

impl Solution {
    pub fn _func(s: String, word_dict: &Vec<String>, dp: &mut HashMap<usize, bool>) -> bool {
        if s.len() == 0 {
            return true;
        }
        
        if let Some(b) = dp.get(&s.len()) {
            return *b;
        }
        
        for prefix in word_dict {
            match s.strip_prefix(prefix) {
                Some(tail) => {
                    if Self::_func(tail.to_string(), word_dict, dp) {
                        dp.insert(s.len(), true);
                        return true;
                    }
                },
                None => {}
            }
        }
        dp.insert(s.len(), false);
        false
    }

    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = HashMap::new();
        dp.insert(0, true);
        Self::_func(s, &word_dict, &mut dp)
    }
}
