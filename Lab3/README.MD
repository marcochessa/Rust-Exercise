# Lab 3
## Exercise 1 - DNA

In this exercise, we will tackle some examples of multiple searches within a data structure to familiarize ourselves with the use of lifetimes in Rust, lambda functions, and iterators.

Not all examples will compile, and the goal is to understand why the compiler disallows certain seemingly valid operations.

In the `es301.rs` file, several functions are provided without lifetime annotations and implementations. These need to be annotated with the correct lifetimes and completed to achieve the expected results from the provided usage examples.

### Suggestion:
Comment out the entire file and uncomment one function at a time to focus on a single problem, avoiding the need to solve all compilation issues first.

### Subsequence Search (Functions subsequence1-3)
The first set of functions requires searching for DNA subsequences within strings that represent DNA chains. DNA is a long chain of molecules of four types, denoted by the symbols A, C, G, and T; thus, a DNA sequence can be represented by a string with repeated characters ACGT.

A specific subsequence can be synthetically represented by a string like `“A1-2,T1-3,A2-2,G2-4,C2-2”`, where the number represents the minimum and maximum repetitions of the corresponding base. For example, this string requires searching for a variable length sequence with 1-2 A, 1-3 T, 2 A, 2-4 G, 2 C (e.g., `ATAAGGCC` but also `AATTAAGGGCC`).

### Implementing Subsequence Search
Three alternative ways to iterate over the found subsequences are required:
- A lambda function passed to the search function and called for each found subsequence (`subsequence4`)
- A naive iterator implemented via a struct, allowing the search to be interrupted at any moment (`SimpleDNAIter`)
- A Rust-compliant iterator (`DNAIter`)
- An iterator created by a generator, without the need to define a support struct (`subsequence5_iter`)

See the examples in `es0301.rs` and the comments that guide you to the solution.

## Exercise 2 - FileSystem
## FileSystem Struct

Implement a `FileSystem` struct to manage the structure (names and relationships) of a filesystem in memory, offering operations for creation, removal, search, and updating of folders and files.

The interface to be implemented is provided in the `es0302.rs` file, including necessary guidelines and a usage example. To compile and run the code correctly, functions and methods will need to be annotated with appropriate lifetimes.

### Suggestion:
Comment out everything and add one piece at a time to avoid fixing all compilation errors at once.

### Note on Mutable References
Pay particular attention to the use of the `find` function in the usage example, when trying to get a `get_mut()` of the found path. It will not compile and cannot be made to compile without a different approach. Explain the reason clearly, tracing the lifetimes of the involved variables.

Subsequently, comment out that piece of code and try to obtain a mutable reference from the found paths using the suggestion in the code.

### Theory Note
Notice that the filesystem structure is a tree, implemented without using a pointer but only a standard collection `Vec`.

This might suggest that a binary tree with a node defined like this should work:
```rust
struct Node {
    val: i32,
    left: Node,
    right: Node,
}
```

If you try to compile this, Rust will give a compilation error. What is the error and how do you explain it? Conversely, defining the `Node` structure like this (as used in `FileSystem`) works:
```rust
struct Node {
    val: i32,
    children: Vec<Node>,
}
```

What is the difference between the first and second examples? Where is the data allocated?