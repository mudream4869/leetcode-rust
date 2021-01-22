impl Solution {
    pub fn generate(n_i32: i32) -> Vec<Vec<i32>> {
        let n = n_i32 as usize;
        let mut ret_v: Vec<Vec<i32>> = Vec::with_capacity(n);
        for i in 0..n {
            let mut ret_vv: Vec<i32> = Vec::with_capacity(i+1);
            for j in 0..i+1 {
                if j == 0 || j == i {
                    ret_vv.push(1);
                } else {
                    ret_vv.push(ret_v[i-1][j] + ret_v[i-1][j-1]);
                }
            }
            ret_v.push(ret_vv);
        }
        ret_v
    }
}
