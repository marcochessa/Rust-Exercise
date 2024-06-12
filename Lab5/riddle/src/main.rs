use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use clap::Parser;
use itertools::{Itertools, repeat_n};

#[derive(Parser, Debug)]
struct Args {
    num_string: String, // input string
}

const OPERATIONS: [char; 4] = ['+', '-', '/', '*'];

fn generate_tuple(digits: &mut Vec<i32>) -> Vec<(Vec<i32>, Vec<char>)> {

    // Generate all permutations of 5 digits from the input vector.
    // The `cloned` method converts `&i32` references to `i32` values.
    let digits_perms = digits.iter().cloned().permutations(5);
    println!("Perm {}", digits_perms.try_len().unwrap());

    // Generate all possible combinations of 4 operations from the OPERATIONS array.
    let op_combs = repeat_n(OPERATIONS.clone(), 4).multi_cartesian_product();
    println!("Comb {}", op_combs.try_len().unwrap());

    // Create a vector of tuples by taking the cartesian product of digit permutations and operation combinations.
    let result: Vec<(Vec<i32>, Vec<char>)> = digits_perms.cartesian_product(op_combs)
        .map(|(num, ops)| (num, ops))
        .collect();

    result
}

fn check_tuple(tuple: (Vec<i32>, Vec<char>)) -> bool {
    // Ensure both vectors have the right length
    if tuple.0.len() - 1 != tuple.1.len() {
        return false;
    }

    // Initialize num with the first element of the numeric vector
    let mut num = tuple.0[0];

    // Iterate over the rest of the elements
    for (i, el) in tuple.0.iter().enumerate().skip(1) {
        match tuple.1[i - 1] {
            '+' => num += el,
            '-' => num -= el,
            '*' => num *= el,
            '/' => {
                // Handle division by zero
                if *el == 0 {
                    return false;
                } else {
                    num /= el;
                }
            }
            _ => {
                // Handle invalid characters
                return false;
            }
        }
    }
    num == 10
}

fn main() {
    /* UTILIZZO ARGS CON CLAP */

    let args = Args::parse();

    let mut digits: Vec<i32> = vec![];
    args.num_string.split(" ").for_each(|n| digits.push(n.parse().unwrap()));


    let n_threads = 10;

    let vec_tuple = generate_tuple(&mut digits);
    println!("Tuple: {}",vec_tuple.len());

    let data = vec_tuple.chunks(vec_tuple.len()/n_threads);
    println!("Chunks and Threads number: {}",data.len());


    let mut results: Vec<(Vec<i32>, Vec<char>)> = vec!();


    let start = Instant::now();
    thread::scope(|s| {
        let mut threads = vec![];
            for chunk in data {
                let thread = s.spawn(move || {
                    let mut v = vec!();
                    for tuple in chunk {
                        if check_tuple(tuple.clone()) {
                            v.push(tuple.clone());
                        }
                    }
                    return v;
                });
                threads.push(thread);
            }
            for t in threads {
                results.append(&mut t.join().unwrap());
            }
    });
    println!("valid sequence: {}, elapsed time: {:?}",results.len(), start.elapsed().as_nanos());

}
