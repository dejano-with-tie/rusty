use std::thread::JoinHandle;

pub struct ThreadPool {
    pub workers: Vec<Worker>,
}

pub struct Worker {
    pub id: usize,
    pub thread: JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize) -> Worker {
        let thread = std::thread::spawn(|| {});
        Worker { id, thread }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i));
        }

        ThreadPool { workers }
    }
}
