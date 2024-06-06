// find all subsequences of seq in s and return a vector of tuples containing the start position
// and the found subsequences as string slices
// ignore overlaps: if a subsequence is found, the search must continue from the next character
// missing lifetimes: the result string slices depend only from one input parameter, which one?

// suggestion: write a function find_sub(&str, &str) -> Option<(usize, &str)> that finds the first subsequence in a string, you can use it in all the following functions

use std::{char, collections::VecDeque};


// let's define a struct to store the search token A1-2, C1-1, etc.
#[derive(Debug, Clone)]
struct SearchToken(char, usize, usize);

impl SearchToken {
    fn parse_str(seq: &str) -> Vec<Self> {
        seq.split(",")
            .map(|t| {
                let base = t.chars().nth(0).unwrap();
                let range = t[1..]
                    .split("-")
                    .map(|s| usize::from_str_radix(s, 10).unwrap())
                    .collect::<Vec<usize>>();
                SearchToken(base, range[0], range[1])
            })
            .collect::<Vec<SearchToken>>()
    }
}

enum ParseState {
    Advance,
    CheckSequence,
}

// return a match only if the subsequence exactly matches the search pattern: e.g. A1-1,C1-2 does not match AACC, but only AC or ACC
// a solution matching open sequences at start and at end (eg. matching also A[ACC]CC) would have been equally acceptable  
fn find_sub<'a>(s: &'a str, seq: &Vec<SearchToken>) -> Option<(usize, &'a str)> {
    
    // make the itertator peekable, so than we can check next char without consuming it
    let mut it = s.chars().peekable();
    let mut char_runs = VecDeque::new();
    let mut state = ParseState::Advance;
    let mut start_pos = 0;

    // we have two states:
    // Advance: we count the next char run (same chars) and we store it; when we have enough runs we check the sequence
    // CheckSequence: we check if the current char run is ok, otherwise we advance of one run
    loop {
        match state {
            ParseState::Advance => {
                let c = if let Some(c) = it.next() {
                    c
                } else {
                    // not more chars and no sequence found
                    return None;
                };
                let mut count = 1;
                // count chars using peek
                while let Some(&next_c) = it.peek() {
                    if next_c == c {
                        count += 1;
                        it.next();
                    } else {
                        // changed char, stop counting
                        break;
                    }
                }
                char_runs.push_back((c, count));
                if char_runs.len() == seq.len() {
                    state = ParseState::CheckSequence;
                }
            }
            ParseState::CheckSequence => {
                let mut found = true;
                let mut len = 0;
                for (i, (c, count)) in char_runs.iter().enumerate() {
                    len += count;
                    let SearchToken(seq_c, seq_min, seq_max) = seq[i];
                    if *c != seq_c || *count < seq_min || *count > seq_max {
                        found = false;
                        break;
                    }
                }

                let (_, count) = char_runs.pop_front().unwrap();
                if found {
                    return Some((
                        start_pos as usize,
                        &s[start_pos..start_pos + len],
                    ));
                } else {
                    start_pos += count;
                    state = ParseState::Advance;
                }
            }
        }
    }
}

pub fn run_test_find() {
    let a = "ACTCCCAACCGGTACACCCC".to_string();
    // let a = "AC".to_string(); // test no match and short sequence
    let seq = "A1-1,C2-4";

    let toks = SearchToken::parse_str(seq);
    println!("toks: {:?}", toks);
    let res = find_sub(&a, &toks);

    match res {
        Some((off, sub)) => {
            println!("Found subsequence at position {}: {}", off, sub);
        }
        None => {
            println!("No subsequence found");
        }
    }
}

fn subsequences1<'a>(s: &'a str, seq: &str) -> Vec<(usize, &'a str)> {
    let mut res = Vec::new();
    let mut pos = 0;
    let search_tokens = SearchToken::parse_str(seq);

    while let Some((mstart, smatch)) = find_sub(&s[pos..], &search_tokens) {
        pos = mstart + smatch.len();
        res.push((mstart, smatch));
    };
    res
}

pub fn demo1() {
    // removed an A to show at least a match
    //let a = "AACGGTAACC".to_string();
    let a = "AACGGTACC".to_string();
    let seq = "A1-1,C2-4";

    for (off, sub) in subsequences1(&a, seq) {
        println!("Found subsequence at position {}: {}", off, sub);
    }
}

// Now we want to find different subsequences at the same time, seq is a vector of string slices with many subsequence to search
// For each subsequence find all the matches and to the results (there may be overlaps, ignore them), but in this way you can reuse the previous solution
// The result will contain: the start position in s, the found subsequence as string slice and the mached subsequence in seq
// Now the string slices in the rsult depend from two input parameters, which ones?
fn subsequences2<'a, 'b>(s: &'a str, seq: &[&'b str]) -> Vec<(usize, &'b str, &'a str)> {
    let mut res = Vec::new();
    for seq_i in seq {
        for (start, smatch) in subsequences1(s, seq_i) {
            res.push((start, *seq_i, smatch));
        }
    }
    res
}

pub fn demo2() {
    let a = "AACGTTACC".to_string();
    let seqs = ["A1-1,C2-4", "G1-1,T2-4"];

    for (off, matched, sub) in subsequences2(&a, &seqs) {
        println!("Found subsequence {} at position {}: {}", matched, off, sub);
    }
}

// Now we want to do some DNA editing! Therefore we receive a mutable string and we'd like to return a vector of mutable string slices
// Follow this steps:
// 1. adjust the lifetimes without any implementation yet: does it compile?
// 2. try to implement the function: does it compile?
// 3. if it doesn't compile, try to understand why from the compiler errors and draw all the necessary lifetimes
// 4. Spoiler: basically it's not possibile to return more then one mutable reference to the same data
// 5. Try this workaround: return a vector of indexes (first solution) and let the caller extract the mutable references
// 7. (later in the course you will learn about smart pointers, which can be used to solve this kind of problems in a more elegant way)

// SOLUTION 1.: this compiles 
//fn subsequences3<'a>(s: &'a mut str, seq: &str) -> Vec<(usize, &'a mut str)> {
//    let mut v = Vec::new();
//    v
//}


// SOLUTION 2..4: as soon as you try to get a second mutable reference from s (in the for loop) rust stops you.
// as work around use indexes of the matches: (start, end) of each pos
fn subsequences3(s: &str, seq: &str) -> Vec<(usize, usize)> {
    let mut v = Vec::new();
    for (start, smatch) in subsequences1(s, seq) {
        v.push((start, start + smatch.len()));
    }
    v
}

pub fn demo3() {
    let mut a = "AACGGTACC".to_string();
    let seq = "A1-1,C2-4";

    for (start, end) in subsequences3(&a, seq) {
        println!("Found subsequence at position {} - {}", start, end);
        // here we don't have any reference to the search strin a! 
        let mut s = &mut a[start..end];
        // here we can use s as mutable!!! why? because the lifetime of this mutable reference is just one loop iteration
    }
}

// DNA strings may be very long and we can get a lot of matches.
// Therefore we want to process a subsequence as soon as we find it, without storing it in a vector
// A solution is to pass a closure to the function, which will be called for each match
// do you need to put lifetime annotations in the closure? why?
fn subsequence4(s: &str, seq: &str, f: impl Fn(usize, &str)) {
    let mut pos = 0;
    let search_tokens = SearchToken::parse_str(seq);
    while let Some((start, smatch)) = find_sub(&s[pos..], &search_tokens) {
        pos = start + smatch.len();
        f(start, smatch);
    }
}

pub fn demo4() {
    let a = "AACGGTACC".to_string();
    let seq = "A1-1,C2-4";

    subsequence4(&a, seq, |pos, sub| {
        println!("Found subsequence at position {}: {}", pos, sub);
    });
}

// Now let's define a struct SimpleDNAIter (add the required lifetimes), memorizing a DNA sequence and the subsequence to search
// Then we add a next() method to the struct, which will return the next subsequence found in the DNA sequence after each call
// The result of next() is a tuple, but it's wrapped in an Option, because a call to next() may find no more subsequences in the DNA sequence
// In order to implement it, you may add any other attribute to the struct (remember: the struct is stateful and after each call to next() you must start from the last position found)
// The struct may be used as shown in the demo_SimpleDNAIter() function
// This approach is similar to the previous one, but it's more flexible and it can be used in more complex scenarios. For example you may interrupt it
// at any time and resume it later

// attributes must live as long as the struct
struct SimpleDNAIter<'a> {
    s: &'a str,
    pos: usize,
    search_tokens: Vec<SearchToken>
}

impl<'a> SimpleDNAIter<'a> {
    pub fn new(s: &'a str, seq: &'a str) -> Self {
        SimpleDNAIter { s, pos: 0, search_tokens: SearchToken::parse_str(seq) }
    }

    // self must be mut, since we need to advance pos!
    pub fn next(&mut self) -> Option<(usize, &str)> {
        if let Some((start, smatch)) = find_sub(&self.s[self.pos..], &self.search_tokens) {
            let _start = self.pos + start;
            self.pos = self.pos + start + smatch.len();
            return Some((_start, smatch));
        } 
        None
    }
}

pub fn demo_simple_dnaiter() {
    
    // the iterator must become mut in order to adavnce
    let mut dna_iter = SimpleDNAIter::new("ACGTACGTACCGTACCGT", "A1-1,C1-2");

    while let Some((pos, subseq)) = dna_iter.next() {
        println!("Found subsequence at position {}: {}", pos, subseq);
        // we can break and stop if we have found what we were looking for
    }
}

// finally we want to implement a real iterator, so that it can be used in a for loop and it may be combined we all the most common iterator methods
// The struct DNAIter is already defined, you have to implement the Iterator trait for it and add lifetimes
struct DNAIter<'a> {
    s: &'a str,
    pos: usize,
    search_tokens: Vec<SearchToken>
}

impl<'a> DNAIter<'a> {
    pub fn new(s: &'a str, seq: &'a str) -> Self {
        DNAIter { s, pos: 0, search_tokens: SearchToken::parse_str(seq) }
    }
}

impl<'a> Iterator for DNAIter<'a> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((start, smatch)) = find_sub(&self.s[self.pos..], &self.search_tokens) {
            let _start = self.pos + start;
            self.pos = _start + smatch.len();
            return Some((_start, smatch));
        } 
        None
    }
}

pub fn demo_dna_iter() {
    let dna_iter = DNAIter::new("ACGTACGTAAACCGTACGT", "A1-3,C1-2");

    // now you can combine it with all the iterator modifiers!!!
    dna_iter
        .filter(|(_, sub)| sub.len() >= 5)
        .for_each(|(pos, sub)| {
            println!(
                "Found subsequence at least long 5 at position {}: {}",
                pos, sub
            )
        });
}

// now let's return an iterator without defining a struct, just using a closure
// the std lib of rust support you with the std::from_fn() function
// we supply a skeleton implementation, you have to fill the closure
fn subsequence5_iter<'a>(s: &'a str, seq: &str) -> impl Iterator<Item = (usize, &'a str)> {
    let mut pos = 0;
    let search_tokens = SearchToken::parse_str(seq);

    // and any other necessary variable to remember the state
    std::iter::from_fn(move || {
        if let Some((start, smatch)) = find_sub(&s[pos..], &search_tokens) {
            let _start = pos + start;
            pos = _start + smatch.len();
            return Some((_start, smatch));
        } 
        None
    })
}

pub fn demo_dna_iter2() {
    subsequence5_iter("ACGTACGTAAACCGTACGT", "A1-3,C1-2")
        .filter(|(_, sub)| sub.len() >= 5)
        .for_each(|(pos, sub)| {
            println!(
                "Found subsequence at least long 5 at position {}: {}",
                pos, sub
            )
        });
}
