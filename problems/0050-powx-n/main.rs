fn my_pow_1(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }

    if n == 1 {
        return x;
    }

    let half = my_pow_1(x, n/2);
    if n%2 == 0 {
        return half*half;
    }

    half*half*x
}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n > 0 {
            return my_pow_1(x, n);
        }
        
        1.0/my_pow_1(x, -n)
    }
}
