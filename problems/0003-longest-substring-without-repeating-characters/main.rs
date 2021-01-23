use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut st: usize = 0;
        let mut ed: usize = 0;
        // [st, ed)
        
        let mut set: HashSet<char> = HashSet::new();
        let mut ans: usize = 0;        
        while ed < s.len() {
            let toPush = s.chars().nth(ed).unwrap();
            if set.contains(&toPush) {
                let toPop = s.chars().nth(st).unwrap();
                set.remove(&toPop);
                st += 1;
                continue;
            }
            
            ed += 1;
            set.insert(toPush);
            if set.len() > ans {
                ans = set.len();
            }
        }
        
        ans as i32
    }
}
