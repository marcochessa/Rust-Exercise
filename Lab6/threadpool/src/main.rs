use std::sync::{Arc, Mutex, mpsc::{self, Sender, Receiver}};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

type Job = Box<dyn FnOnce() + Send>;

enum JobMessage {
    NewJob(Job),
    WorkerDone(usize),
    Stop,
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Receiver<JobMessage>, event_sender: Sender<JobMessage>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.recv().unwrap();
            match message {
                JobMessage::NewJob(job) => {
                    job();
                    event_sender.send(JobMessage::WorkerDone(id)).unwrap();
                }
                JobMessage::Stop => {
                    event_sender.send(JobMessage::WorkerDone(id)).unwrap();
                    return
                },
                _ => (),
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
    event_sender: Sender<JobMessage>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        let (event_sender, event_receiver) = mpsc::channel();


        let job_queue = Vec::new();
        let mut free_workers =  Vec::new();

        let mut workers = Vec::with_capacity(size);
        let mut job_senders = Vec::with_capacity(size);
        for id in 0..size {
            let (job_sender, job_receiver) = mpsc::channel();
            job_senders.push(job_sender);
            workers.push(Worker::new(id, job_receiver, event_sender.clone()));
            free_workers.push(id);
        }

        let mut scheduler = Scheduler {
            job_queue,
            free_workers,
            job_senders,
            event_receiver,
            stop_received: false
        };

        thread::spawn(move || scheduler.run());

        ThreadPool {
            workers,
            event_sender,
        }
    }

    fn execute(&self, job: Job) {
        self.event_sender.send(JobMessage::NewJob(job)).unwrap();
    }

    fn stop(self) {
        self.event_sender.send(JobMessage::Stop).unwrap();
    }
}

struct Scheduler {
    job_queue: Vec<Job>,
    free_workers: Vec<usize>,
    job_senders: Vec<Sender<JobMessage>>,
    event_receiver: Receiver<JobMessage>,
    stop_received: bool
}

impl Scheduler {
    fn run(&mut self) {
        self.stop_received = false;
        loop {
            let event = self.event_receiver.recv().unwrap();
            match event {
                JobMessage::NewJob(job) => {
                    if let Some(id) =  self.free_workers.pop() {
                        self.job_senders[id].send(JobMessage::NewJob(job)).unwrap();
                    } else {
                        self.job_queue.push(job);
                    }
                }
                JobMessage::WorkerDone(id) => {
                    if let Some(job) = self.job_queue.pop() {
                        self.job_senders[id].send(JobMessage::NewJob(job)).unwrap();
                    } else {
                        self.free_workers.push(id);
                        if self.stop_received && self.free_workers.len() == self.job_senders.len() {
                            return;
                        }

                    }
                }
                JobMessage::Stop => {
                    self.stop_received = true;
                    for sender in self.job_senders.iter(){
                        sender.send(JobMessage::Stop).unwrap();
                    }
                    if self.free_workers.len() == self.job_senders.len() {
                        return;
                    }
                },
            }
        }
    }
}

fn main() {
    let threadpool = ThreadPool::new(10);

    for x in 0..100 {
        threadpool.execute(Box::new(move || {
            println!("long running task {}", x);
            thread::sleep(Duration::from_millis(1000));
        }));
    }

    // Simulate some work in the main thread
    thread::sleep(Duration::from_millis(5000));

    // Stop the thread pool and wait for all workers to finish
    threadpool.stop();
}
