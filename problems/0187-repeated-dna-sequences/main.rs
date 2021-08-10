use std::collections::HashMap;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        if s.len() < 10 {
            return Vec::new();
        }
        
        let mut table: HashMap<String, usize> = HashMap::new();
        
        for i in 0..(s.len() - 9) {
            let counter = table.entry(s[i..(i+10)].to_string()).or_insert(0);
            *counter += 1;
        }
        
        let mut ans = Vec::new();
        for (k, v) in table {
            if v > 1 {
                ans.push(k);
            }
        }
        
        ans
    }
}
