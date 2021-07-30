impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        fn str2vec(s: &String) -> Vec<bool> {
            let mut ret = Vec::new();
            for c in s.chars().rev() {
                ret.push(c == '1');
            }
            ret
        }
        
        fn vec2str(v: &Vec<bool>) -> String {
            let mut ret = String::from("");
            for i in (0..v.len()).rev() {
                if v[i] {
                    ret.push('1');
                } else {
                    ret.push('0');
                }
            }
            ret
        }
        
        let avec = str2vec(&a);
        let bvec = str2vec(&b);
        let mut ans = Vec::new();
        let mut one = 0;
        for i in 0..std::cmp::max(avec.len(), bvec.len()) {
            let mut s = one;
            if i < avec.len() && avec[i] {
                s += 1;
            }
            if i < bvec.len() && bvec[i] {
                s += 1;
            }
            
            if s >= 2 {
                s -= 2;
                one = 1;
            } else {
                one = 0;
            }
            
            ans.push(s == 1);
        }
        
        if one == 1 {
            ans.push(true);
        }
        
        vec2str(&ans)
    }
}
