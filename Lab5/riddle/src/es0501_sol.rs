
use itertools::{self, Itertools, repeat_n};
use std::{collections::HashSet, thread, time::{Instant, Duration}};


fn mk_ops() -> Vec<Vec<char>> {
    let mut res = Vec::new();
    let ops = ['+', '-', '*', '/'];
    for a in repeat_n(ops, 4).multi_cartesian_product() {
        res.push(a);
    }
    res
}

type Combination = (Vec<i32>, Vec<char>);

fn prepare(input: &str) -> Vec<Combination> {
    let mut result = Vec::new();
    for v in input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .permutations(5)
    {
        for ops in mk_ops() {
            result.push((v.clone(), ops));
        }
    }

    result
}

fn verify(v: &[Combination]) -> Vec<String> {
    let mut result = Vec::new();

    'line_chek: for (nums, ops) in v {
        let mut res = nums[0];
        for i in 1..nums.len() {
            if let Some(_res) = match ops[i - 1] {
                '+' => Some(res + nums[i]),
                '-' => Some(res - nums[i]),
                '*' => Some(res * nums[i]),
                '/' => {
                    if nums[i] == 0 {
                        None
                    } else  {
                        res.checked_div(nums[i])
                    } 
                }
                _ => panic!("invalid operator"),
            } {
                res = _res;
            } else {
                continue 'line_chek;
            }
        }

        if res == 10 {
            let mut s = String::new();
            let mut it_op = ops.iter();
            for num in nums.iter() {
                s.push(('0' as u8 + *num as u8) as char);
                if let Some(op) = it_op.next() {
                    s.push(*op);
                }
            }
            result.push(s);
        }
    }

    result
}

pub fn measure_speed() {

    let s = "12270";
    let v = prepare(&s);


    for nthreads in 1..12 {
        let start = Instant::now();

        // please note that the thread::scope is necessary in order to keep the refereces to the chunks alive
        // otherwise the compiler will complain that the lifetime of the vector is too short
        let results = thread::scope(|s| {
            let mut threads = Vec::new();
            for i in 0..nthreads {
                let batch = v.len() / nthreads;
                let slice = if i < nthreads - 1 {
                    &v[i * batch..(i + 1) * batch]
                } else {
                    &v[i * batch..]
                };
                threads.push(s.spawn(move || verify(slice)));
            }

            let mut results = HashSet::new();
            for t in threads {
                for rs in t.join().unwrap() {
                    results.insert(rs);
                }
            }
            results
        });
        println!("[{} threads] elapsed: {:?}", nthreads, start.elapsed().as_nanos());
    }

}
