fn xor3(mut a: u32, mut b: u32) -> u32 {
    // 1 + 1 + 1 = 0
    let mut ret = 0;
    let mut p3 = 1;
    while a > 0 || b > 0 {
        let i = (a%3 + b%3)%3;
        ret += i*p3;
        p3 *= 3;
        a /= 3;
        b /= 3;
    }
    return ret;
}

fn i2u(a: i32) -> u32 {
    (a as i64 + i32::min as i64) as u32
}

fn u2i(a: u32) -> i32 {
    (a as i64 - i32::min as i64) as i32
}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ret_u32 = 0;
        for n in &nums {
            ret_u32 = xor3(ret_u32, i2u(*n));
        }
        u2i(ret_u32)
    }
}
