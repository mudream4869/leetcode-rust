impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        fn find(tab: &mut Vec<usize>, nums: &mut Vec<bool>, n: usize, ans: &mut Vec<Vec<usize>>) {
            if tab.len() == n {
                ans.push(tab.to_vec());
                return;
            }
            
            let cur = tab.len();
            
            for i in 0..n {
                if nums[i] {
                    continue;
                }
                
                let mut ok = true;
                for j in 0..cur {
                    if (cur - j) == (i as i32 - tab[j] as i32).abs() as usize {
                        ok = false;
                        break;
                    }
                }
                
                if ok {
                    nums[i] = true;
                    tab.push(i);
                    find(tab, nums, n, ans);
                    tab.pop();
                    nums[i] = false;
                }
            }
        }
        
        let mut tab = Vec::new();
        let mut nums = vec![false; n as usize];
        let mut ans = Vec::new();

        find(&mut tab, &mut nums, n as usize, &mut ans);
        
        let mut ret = Vec::new();
        for status in ans.iter() {
            let mut lines = Vec::new();
            for pos in status.iter() {
                let mut tmp = vec!['.' as u8; n as usize];
                tmp[*pos] = 'Q' as u8;
                lines.push(String::from_utf8(tmp).unwrap());
            }
            ret.push(lines);
        }
        
        ret
    }
}
