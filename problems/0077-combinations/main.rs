impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut anss: Vec<Vec<i32>> = vec![vec![]];
        for _ in 0..k {
            let mut new_anss: Vec<Vec<i32>> = vec![];
            for ans in anss {
                for to_add in 1..(n+1) {
                    if ans.len() > 0 && ans[ans.len() - 1] >= to_add {
                        continue;
                    }

                    let mut ok = true;
                    for i in &ans {
                        if *i == to_add {
                            ok = false;
                            break;
                        }
                    }
                    if !ok {
                        continue;
                    }
                    let mut new_ans = ans.to_vec();
                    new_ans.push(to_add);
                    new_anss.push(new_ans);
                }
            }
            anss = new_anss;
        }
        anss
    }
}
