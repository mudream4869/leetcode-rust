impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let inf = 100000;
        let mut tab = vec![inf; n];
        tab[n-1] = 0;
        
        for i in (0..(n-1)).rev() {
            for j in i..(i + 1 + nums[i] as usize) {
                if j >= n {
                    break;
                }
                tab[i] = std::cmp::min(tab[i], tab[j]+1);
            }
        }
        
        tab[0]
    }
}
