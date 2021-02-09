fn solve(index: usize, arr: &Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    if index == arr.len() {
        if target != 0 {
            return vec![];
        }
        return vec![vec![]];
    }
        
    let mut ans = vec![];
    
    let val = arr[index];
    
    // [0, target/arr[index]]
    for i in 0..(target/val + 1) {
        let mut sub_anses = solve(index+1, arr, target - val*i);
        for sub_ans in sub_anses.iter_mut() {
            for j in 0..i {
                sub_ans.push(val);
            }
        }
        ans.extend(sub_anses);
    }
    
    ans
}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        solve(0, &candidates, target)
    }
}
