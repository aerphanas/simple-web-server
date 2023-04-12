mod worker;

use std::sync::{
    mpsc::{self, Sender},
    Arc, Mutex,
};

use worker::Worker;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        (0..size).for_each(|num| threads.push(Worker::new(num, Arc::clone(&receiver))));
        ThreadPool {
            workers: threads,
            sender: Some(sender),
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        self.workers.iter_mut().for_each(|f| {
            println!("Shutting down worker {}", f.id);
            if let Some(thread) = f.thread.take() {
                thread.join().unwrap()
            }
        });
    }
}
