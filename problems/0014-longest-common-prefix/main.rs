impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut bytes : Vec<&[u8]> = Vec::new();
        let scnt = strs.len();
        for s in strs.iter() {
            bytes.push(s.as_bytes());
        }
        
        let mut ans = 0usize;
        for i in 0usize..200usize {
            let mut ok = true;
            for idx in 0..scnt {
                if bytes[idx].len() <= i || bytes[idx][i] != bytes[0][i]{
                    ok = false;
                    break;
                }
            }
            
            if !ok {
                break;
            }
            ans = i + 1;
        }
        
        return String::from(&strs[0][0..ans]);
    }
}
                                       
