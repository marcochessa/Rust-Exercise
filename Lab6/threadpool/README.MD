# ThreadPool Implementation in Rust

This repository contains a simple implementation of a thread pool in Rust. The thread pool allows for the execution of multiple tasks in parallel using a fixed number of worker threads. This is useful for efficiently managing the execution of many small tasks without the overhead of creating a new thread for each task.

## Features

- Fixed-size thread pool
- Task scheduling and execution
- Graceful shutdown of workers
- Safe concurrency using channels and mutexes
- 
### Usage

1. Clone this repository:

2. Build the project:

```sh
cargo build
```

3. Run the example:

```sh
cargo run
```


### Explanation

- **Worker**: Each worker in the pool runs in its own thread and listens for jobs on a channel. When a job is received, it executes the job and then signals that it is done.
- **ThreadPool**: Manages a pool of workers and a scheduler. It provides an interface to execute jobs and stop the pool.
- **Scheduler**: Manages the assignment of jobs to free workers and handles the shutdown process.
