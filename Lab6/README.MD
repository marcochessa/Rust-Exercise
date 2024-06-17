# Lab 6

## Exercise 1: Cyclic Barrier with Channels

Implementing a cyclic barrier using channels for synchronization between threads, without shared state in a mutex. Each barrier instance manages `n` `Waiter` objects, each containing `n-1` senders and 1 receiver. The `wait()` method on `Waiter` sends messages through its channels and waits to receive messages back.

### Example Invocation:

```rust
fn main() {
    let barrier = cb::CyclicBarrier::new(3);
    let mut threads = Vec::new();

    for i in 0..3 {
        let waiter = barrier.get_waiter();
        threads.push(std::thread::spawn(move || {
            for j in 0..10 {
                waiter.wait();
                println!("after barrier {} {}", i, j);
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
```

## Exercise 2: Thread Pool

A thread pool allocates a fixed number of threads and executes jobs received through an `execute` method. Jobs are executed asynchronously in worker threads.

### Features:

- Jobs are executed in existing worker threads.
- Queues jobs when all workers are busy.
- Supports fire-and-forget execution; jobs do not return values but can include mechanisms for result retrieval (e.g., barriers or channels).

### Implementation:

The `execute` method sends jobs through a channel to an internal scheduler, which assigns them to available workers. When a job finishes, the scheduler assigns the next job in the queue to a free worker.

### Example Usage:

```rust
fn main() {
    // Allocate workers
    let thread_pool = ThreadPool::new(10);

    // Submit jobs to the pool
    for x in 0..100 {
        thread_pool.execute(Box::new(move || {
            println!("long running task {}", x);
            thread::sleep(Duration::from_millis(1000))
        }))
    }

    // Keep the main thread alive
    loop {
        thread::sleep(Duration::from_millis(1000));
    }
}
```

### Bonus:

Add a `stop()` method to the thread pool to halt all workers and wait for their completion, executing any queued jobs before stopping.

## Exercise 3: Downloader - Processes

Demonstrates the advantages of using child processes over threads for tasks such as network operations, where unpredictable durations can lead to blocking or timeouts.

### Downloader Object:

Downloads a URL with a specified timeout using a child process (e.g., `curl`). If the timeout expires, the child process is terminated to free resources.

### Example Usage:

```rust
fn main() {
    let downloader = Downloader::new("http://www.google.com", 10);
    match downloader.start() {
        Ok(data) => {
            println!("Downloaded data: {} bytes", data.len());
        }
        Err(e) => match e {
            DownloaderError::Timeout => println!("Error: Timeout occurred"),
            DownloaderError::Other(err) => println!("Error: {}", err),
        },
    }
}
```