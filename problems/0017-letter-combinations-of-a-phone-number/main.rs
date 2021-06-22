use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return vec![];
        }
        let mut digMap = HashMap::new();
        digMap.insert('2', vec!['a', 'b', 'c']);
        digMap.insert('3', vec!['d', 'e', 'f']);
        digMap.insert('4', vec!['g', 'h', 'i']);
        digMap.insert('5', vec!['j', 'k', 'l']);
        digMap.insert('6', vec!['m', 'n', 'o']);
        digMap.insert('7', vec!['p', 'q', 'r', 's']);
        digMap.insert('8', vec!['t', 'u', 'v']);
        digMap.insert('9', vec!['w', 'x', 'y', 'z']);
        let mut anss = vec![String::from("")];
        
        let digCnt = digits.len();
        
        for digit in digits.chars() {
            let mut newAnss = Vec::new();
            match digMap.get(&digit) {
                Some(chars) => {
                    for ch in chars {
                        for ans in &anss {
                            let mut newAns = String::from(ans);
                            newAns.push(*ch);
                            newAnss.push(newAns);
                        }
                    }
                },
                None => {
                    // shouldn't here
                    continue;
                }
            }
            anss = newAnss;
        }
        
        anss
    }
}
      
