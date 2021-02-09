fn solve(n: i32) -> Vec<i32> {
    if n == 1 {
        return vec![0, 1];
    }
    
    let mut ans = solve(n - 1);
    let len = ans.len() as i32;
    let pad = 1 << (n-1);

    for i in 0..len {
        ans.push(ans[(len - i - 1) as usize] + pad);
    }
    
    ans
}

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        solve(n)
    }
}
