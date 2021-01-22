impl Solution {
    pub fn get_row(r_i32: i32) -> Vec<i32> {
        let r = r_i32 as usize;
        let mut arr: Vec<i32> = Vec::new();
        for i in 0..r+1 {
            let mut nxt: Vec<i32> = Vec::with_capacity(i+1);
            for j in 0..i+1 {
                if j == 0 || j == i {
                    nxt.push(1);
                } else {
                    nxt.push(arr[j-1] + arr[j]);
                }
            }
            arr = nxt;
        }
        arr
    }
}
