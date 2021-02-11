fn catalan(n: i64) -> i64 {
    if n == 0{
        return 1;
    }
    
    return 2*(2*n+1)*catalan(n-1)/(n+2);
}


impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        // Catalan number
        catalan((n-1) as i64) as i32
    }
}
