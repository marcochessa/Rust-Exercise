## Lab 5

### Exercise 1: Parallelizing Work

The puzzle requires finding the sequence of elementary operations (addition, subtraction, multiplication, and division) necessary to obtain 10, starting from 5 randomly chosen numbers between 0 and 9. The constraints are:
- The five digits are between 0 and 9 and may repeat.
- The digits must all be used in any order.
- There are no restrictions on the operations (e.g., four additions are fine).
- Operator precedence is not considered; operations are applied from left to right in their order.
- A division by zero or a non-integer result is not a valid operation.

Example: given 27221, a solution could be 7-2-1x2+2 = 10.

Write a program that, given the sequence of digits from the command line as an argument, finds all possible solutions, saves them in a collection of unique strings (e.g., "7 - 2 - 1 x 2 + 2"), and prints them. Use a brute force approach, listing in a vector all possible permutations of the chosen five digits, each combined with all combinations with repetition of the four elementary operations. Compute the result for each and if it's 10, save the sequence; otherwise, discard it.

Since digits and operation symbols are different types, use a vector of tuples to store all possible combinations: the first element is the 5 permuted digits, the second the operations.

Example:
```rust
[
    ([2,7,2,2,1], ['+', '+', '+', '+']), // = 14
    ([2,7,2,2,1], ['+', '+', '+', '-']), // = 12
    ...
    ([1,7,2,2,2], ['+', '+', '+', '+']), // = 14
]
```

Once the program works correctly, use threads to try to speed up the solution search in parallel. Divide the vector of all possible permutations into K equal blocks, and for each block launch a thread that computes the results of the combinations. Try with K=2,3,4... etc., measure the times, and find the number of threads beyond which there are no benefits (if any).

Does anything change if, instead of block division, the work among threads is interleaved? That is, with three threads, the first tries permutations with indices 0,3,6,..., the second 1,4,7,... and the third 2,5,8,...?

If there are no advantages, try with n = 6 or 7 digits instead of 5. Note: the operations are very light, it is likely that large n values are needed to obtain measurable benefits.

Suggestion: to generate permutations and combinations, use the itertools crate and see the permutations() and repeat_n(...).multi_cartesian_product() methods.

### Exercise 2: Resource Sharing Between Threads

Adapt the circular buffer exercise from previous exercises so that the producer (who writes values) and consumer (who reads values) run in two different threads.

To test synchronization, write one value per second into the buffer and read every 2 seconds, then reverse the times. This tests the case of the buffer being moderately full and moderately empty.

Note that in this case, access to the buffer and to the write and read indices must be synchronized, ensuring mutual exclusion between the two threads.

The buffer must therefore be a `struct RingBuf` with two methods, `read() -> Option<T>` and `write(val) -> Result<(),()>` that are thread-safe, i.e., callable within threads without the caller worrying about synchronization operations.

`read` returns `None` when the buffer is empty, while `write` returns `Err(())` when the buffer is full and `Ok(())` when the write was successful.

Bonus: modify producer and consumer to write and read from the buffer without pauses (except when it is full or empty) and measure the throughput of the buffer with different sizes (10, 1000, 10000 values). Also measure the throughput by increasing the number of producers and consumers; how does it vary?

### Exercise 3: Synchronization

A barrier is a synchronization pattern that allows n threads to wait until all of them have reached a common point before proceeding.

The barrier is "closed" until all threads reach a certain point (e.g., a result is ready) and "opens" when the last one arrives at the barrier.

The barrier is initialized with the number of threads expected (n) and when a thread calls `barrier.wait()`, it stops until the last one calls `barrier.wait()`.

A cyclic barrier can be reused. In this case, the threads have a loop that does some work and periodically calls `barrier.wait()`, which always behaves the same.

The barrier is initially closed, then when all threads have arrived, it opens, and when the last one exits, it closes again to block the threads at the next loop.

Attention!!! This implies managing any threads that are too fast, which, once exited, immediately call a second wait() while the barrier is still open because some threads have not yet exited.

(This is a situation that is difficult to debug but must be considered during barrier design; to test it, try introducing random delays on exit from wait)

An example of how it can be used:
```rust
fn main() {
    let abarrrier = Arc::new(cb::CyclicBarrier::new(3));
    let mut vt = Vec::new();
    for i in 0..3 {
        let cbarrier = abarrrier.clone();
        vt.push(std::thread::spawn(move || {
            for j in 0..10 {
                cbarrier.wait();
                println!("after barrier {} {}", i, j);
            }
        }));
    }
    for t in vt {
        t.join().unwrap();
    }
}
```

In this example, the threads advance with their respective indices "j" synchronized, no thread advances faster than the others. Try to see the difference by commenting out `wait()`.

The barrier has two operating states:
- closed: it waits for all threads to arrive
- open: it lets the threads move forward

When it is open on exit, avoid threads calling wait again and re-entering. Ideally, you can think of it as double bank doors: until the outside door is open, the inside door does not open and vice versa.

This implies that a simple counter of how many threads are waiting is not enough; it is also necessary to save a state and signal to the other threads when the barrier opens.

To solve the problem, a condition variable is also needed.

Additionally, try to pass a generic T value to wait and share the value passed among all threads => wait must return a vector with the n collected values.