use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs {
            let mut key = s.as_bytes().to_vec();
            key.sort();
            let strkey = String::from_utf8(key).unwrap();
            groups.entry(strkey).or_insert(vec![]).push(s.to_string());
        }
        
        let mut ans = Vec::new();
        for (_, group) in groups {
            ans.push(group);
        }
        
        ans
    }
}
