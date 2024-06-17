# Lab 4
## Exercise 1

The `es0401.rs` file contains two modules, `List1` and `List2`, with two alternative models for implementing a linked list.

- The first model is based on an `enum`, as discussed in class, with a recursive definition of the list.
- The second model uses a `struct Node` layout with a more C-like implementation approach.

Implement both interfaces as described in the file comments.

### Memory Allocation Analysis:
```rust
let mut l1 = List1::List::<i32>::new();
l1.push(10);

let mut l2 = List2::List::<i32>::new();
l2.push(10);
```

#### Questions:
1. Where are the heads of `l1` and `l2` allocated?
2. What are the differences in the last node between `l1` and `l2`?

### Modifying `List2` to a Doubly-Linked List:
- Add two pointers, one to the head and one to the tail of the list.
- Each node must point to the previous node as well as the next node.
- Implement push and pop operations from both the head and tail of the list.
- Additionally, add the method `fn popn(&mut self, n: usize) -> Option<T>` that removes the nth element of the list and returns it.

#### Considerations:
- Ensure to update `head` or `tail` correctly if the removed node is at the beginning or the end.
- Can `Box` still be used to point to adjacent nodes?
- How should the `Node` structure be modified to have more references?

Refer to the suggestions in the file comments for guidance.

## Exercise 2
## Electric Circuit Description

A file (`es0402_input.txt`) contains the description of an electric circuit with a single generator (G), a series of switches (S), and bulbs (L). Each element is associated with a node and connected upstream to another node. The circuit is guaranteed to be a tree with a single root generator and each node having at most two children.

The file lists nodes in partial order: a parent node always appears before its children, thus the first line is always the generator. The file also describes the initial state of applicable nodes: on/off.

### File Format:
```
type name parent [status]
```

#### Example:
```
G gen1 - off
S sw01 gen1 off
S sw02 gen1 on
L l01 sw01
S sw03 l01 off
L l02 sw03
```

The `es0402.rs` file contains a skeleton interface to set up the problem:
- An `enum NodeFunction` to manage the node's function (generator, switch, bulbs).
- A `struct Node` to store node connections (note that a bulb can have other switches and bulbs downstream, and current flows if the bulb is on).
- A `struct CircuitTree` to store the circuit and manage its functions, including:
    - Building the circuit from the file (via adding a node).
    - Searching for a node by name.
    - Checking if a light is on: a light is on if all switches between the light and the root are closed and the generator is on.
    - Turning a light on/off: to turn on a light, all upstream switches must be turned on.

### Key Points:
- To quickly find a node by name, keep a reference to the node in a hash map: this necessitates using smart pointers with reference counting.
- To efficiently find the path from a node to the root, store a reference to the parent of each node: beware of cyclic references.
- `CircuitTree` implementation does not have any `mut` in the methods for turning lights on: to modify the state of switches, use the interior mutability pattern.