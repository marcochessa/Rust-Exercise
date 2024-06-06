
#[derive(Debug)]
struct BaseSpec {
    base: char,
    min: usize,
    max: usize,
}

fn find_sub<'a, 'b>(s: &'a str, sub: &'b str) -> Option<(usize, &'a str)> {
    let specs: Vec<&str> = sub.split(',').collect();

    let mut spec_vec: Vec<BaseSpec> = vec![];

    for spec in specs {
        let base = spec.chars().nth(0).unwrap();
        let min: usize = spec.chars().nth(1).unwrap().to_digit(10).unwrap() as usize;
        let max: usize = spec.chars().nth(3).unwrap().to_digit(10).unwrap() as usize;
        spec_vec.push(BaseSpec { base, min, max })
    }

    let mut index: usize = 0;
    let mut size: usize = 0;

    for (i, c) in s.chars().enumerate() {
        if c == spec_vec[0].base {
            if size == 0 {
                index = i;
            }
            size += 1;
        } else {
            if size > spec_vec[0].max {
                index += size - spec_vec[0].max;
                size = spec_vec[0].max;
            }
            if size >= spec_vec[0].min {
                let mut sub_index = 1;
                let mut spec_i = 1;

                while spec_i < spec_vec.len() {
                    let current_spec = &spec_vec[spec_i];
                    let mut current_size = 0;
                    let mut found = false;

                    for c in s[index + size..].chars() {
                        if c == current_spec.base {
                            size += 1;
                            current_size += 1;
                            found = true;
                        } else {
                            if current_size >= current_spec.min && current_size <= current_spec.max {
                                sub_index += 1;
                                break;
                            } else {
                                size = 0;
                                break;
                            }
                        }
                    }

                    // If the remaining substring is about to end,
                    // and the current size of the sequence falls within the acceptable range,
                    // increment the subsequence index.
                    if index + size == s.len() {
                        if current_size >= current_spec.min && current_size <= current_spec.max {
                            sub_index += 1;
                        }
                    }

                    if !found || current_size > current_spec.max {
                        size = 0;
                        break;
                    }
                    spec_i += 1;
                }

                if sub_index == spec_vec.len() {
                    return Some((index, &s[index..index + size]));
                }
            }
            size = 0;
        }
    }
    None
}


// find all subsequences of seq in s and return a vector of tuples containing the start position
// and the found subsequences as string slices
// ignore overlaps: if a subsequence is found, the search must continue from the next character
// missing lifetimes: the result string slices depend only from one input parameter, which one?

fn subsequences1<'a, 'b>(s: &'a str, seq: &'b str) -> Vec<(usize, &'a str)> {
    let mut result = Vec::new();
    let mut start = 0;
    while let Some((pos, sub)) = find_sub(&s[start..], seq) {
        result.push((start + pos, sub));
        start += pos + 1;
    }
    result
}

// Now we want to find different subsequences at the same time, seq is a vector of string slices with many subsequence to search
// For each subsequence find all the matches and to the results (there may be overlaps, ignore them), but in this way you can reuse the previous solution
// The result will contain: the start position in s, the found subsequence as string slice and the mached subsequence in seq
// Now the string slices in the rsult depend from two input parameters, which ones?
fn subsequences2<'a, 'b>(s: &'a str, seq: &[&'b str]) -> Vec<(usize, &'a str, &'b str)> {
    let mut result = Vec::new();
    seq.iter().for_each( |&seq_i| {
        if let Some((pos, sub)) = subsequences1(s, seq_i).pop() {
            result.push((pos, sub, seq_i));
        }
    });
    result
}

// Now we want to do some DNA editing! Therefore we receive a mutable string and we'd like to return a vector of mutable string slices
// Follow this steps:
// 1. adjust the lifetimes without any implementation yet: does it compile?
// 2. try to implement the function: does it compile?
// 3. if it doesn't compile, try to understand why from the compiler errors and draw all the necessary lifetimes
// 4. Spoiler: basically it's not possibile to return more then one mutable reference to the same data
// 5. Try this workaround: return a vector of indexes (first solution) and let the caller extract the mutable references
// 7. (later in the course you will learn about smart pointers, which can be used to solve this kind of problems in a more elegant way)
fn subsequences3<'a>(s: &'a mut str, seq: &str) -> Vec<(usize, &'a str)> {
    let mut v = Vec::new();
    /*
    let el = subsequences1(s, seq)[0];
    v.push((el.0, &mut s[el.0..el.1.len()]));
     */
    v = subsequences1(s, seq);
    v
}

// DNA strings may be very long and we can get a lot of matches.
// Therefore we want to process a subsequence as soon as we find it, without storing it in a vector
// A solution is to pass a closure to the function, which will be called for each match
// do you need to put lifetime annotations in the closure? why?
fn subsequence4(s: &str, seq: &str, closure: impl Fn(usize, &str))
{
    let mut start = 0;
    while let Some((pos, sub)) = find_sub(&s[start..], seq) {
        closure(start + pos, sub);
        start += pos + 1;
    }
}

// Now let's define a struct SimpleDNAIter (add the required lifetimes), memorizing a DNA sequence and the subsequence to search
// Then we add a next() method to the struct, which will return the next subsequence found in the DNA sequence after each call
// The result of next() is a tuple, but it's wrapped in an Option, because a call to next() may find no more subsequences in the DNA sequence
// In order to implement it, you may add any other attribute to the struct (remember: the struct is stateful and after each call to next() you must start from the last position found)
// The struct may be used as shown in the demo_SimpleDNAIter() function
// This approach is similar to the previous one, but it's more flexible and it can be used in more complex scenarios. For example you may interrupt it
// at any time and resume it later

// Part 5
struct SimpleDNAIter<'a> {
    s: &'a str,
    seq: &'a str,
    start:usize
}

impl <'a> SimpleDNAIter<'a> {
    pub fn new(s: &'a str, seq: &'a str) -> Self {
        SimpleDNAIter { s, seq, start: 0}
    }

    pub fn next(&mut self) -> Option<(usize, &str)> {
            while let Some((pos, sub)) = find_sub(&self.s[self.start..], self.seq) {
                let absolute_pos = self.start + pos;
                self.start += pos + 1;
                return Some((absolute_pos, sub));
            }
            None
    }
}

// Part 6

// finally we want to implement a real iterator, so that it can be used in a for loop and it may be combined we all the most common iterator methods
// The struct DNAIter is already defined, you have to implement the Iterator trait for it and add lifetimes
struct DNAIter<'a> {
    s: &'a str,
    seq: &'a str,
    start:usize
}

impl <'a> DNAIter <'a>{
    pub fn new(s: &'a str, seq: &'a str) -> DNAIter <'a> {
        DNAIter {
            s,
            seq,
            start:0
        }
    }
}

impl <'a> Iterator  for DNAIter<'a>  {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((pos, sub)) = find_sub(&self.s[self.start..], self.seq) {
            let absolute_pos = self.start + pos;
            self.start += pos + 1;
            return Some((absolute_pos, sub));
        }
        None
    }
}

// Part 7
// now let's return an iterator without defining a struct, just using a closure
// the std lib of rust support you with the std::from_fn() function
// we supply a skeleton implementation, you have to fill the closure
fn subsequence5_iter<'a>(s: &'a str, seq: &'a str) -> impl Iterator<Item = (usize, &'a str)> {
    let mut pos = 0;
    // and any other necessary variable to remember the state
    std::iter::from_fn(move || {
        if let Some(k) = find_sub(&s[pos..], seq) {
                let index = pos + k.0;
                pos = index + 1; // Move position to search for the next subsequence
                Some((index, k.1))
        } else {
            None
        }
    })
}


pub fn main() {
    //demo
}

/*
// DEMO 1
pub fn demo1() {
    let a = "AACGGTAACC".to_string();
    let seq = "A1-1,C2-4";

    for (off, sub) in subsequences1(&a, seq) {
        println!("Found subsequence at position {}: {}", off, sub);
    }
}

// DEMO 2
pub fn demo2() {
    let a = "AACGGTACC".to_string();
    let seqs = ["A1-1,C2-4", "G1-1,T2-4"];

    for (off, matched, sub) in subsequences2(&a, &seqs) {
        println!("Found subsequence {} at position {}: {}", matched, off, sub);
    }
}

// DEMO 3
pub fn demo3() {
    let mut a = "AACGGTAACC".to_string();
    let seq = "A1-1,C2-4";

    for (off, sub) in subsequences3(&mut a, seq) {
        println!("Found subsequence at position {}: {}", off, sub);
    }
}

// DEMO 4
pub fn demo4() {
    let a = "AACGGTAACC".to_string();
    let seq = "A1-1,C2-4";

    subsequence4(&a, seq, |pos, sub| {
        println!("Found subsequence at position {}: {}", pos, sub);
    });
}

// DEMO 5
pub fn demo5() {
    let mut dna_iter = SimpleDNAIter::new("ACGTACGTACGTACGT", "A1-1,C1-1");

    while let Some((pos, subseq)) = dna_iter.next() {
        println!("Found subsequence at position {}: {}", pos, subseq);
        // we can break and stop if we have found what we were looking for
    }
}

// DEMO 6
pub fn demo6() {
    let dna_iter = DNAIter::new("ACGTACGTAAACCCGTACGT", "A1-3,C1-2");

    // now you can combine it with all the iterator modifiers!!!
    dna_iter
        .filter(|(pos, sub)| sub.len() >= 5)
        .for_each(|(pos, sub)| {
            println!(
                "Found subsequence at least long 5 at position {}: {}",
                pos, sub
            )
        });
}

// DEMO 7
pub fn demo7() {
    subsequence5_iter("ACGTACGTAAACCGTACGT", "A1-3,C1-2")
        .filter(|(pos, sub)| sub.len() >= 5)
        .for_each(|(pos, sub)| {
            println!(
                "Found subsequence at least long 5 at position {}: {}",
                pos, sub
            )
        });
}
 */
