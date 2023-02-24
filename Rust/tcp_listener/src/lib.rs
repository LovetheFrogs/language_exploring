use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

/// A structure to manage pools of various threads performing actions.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Creates a new ThreadPool.
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { 
            workers,
            sender: Some(sender),
        }
    }

    /// Exectutes a function using the thread pool.
    /// 
    /// f is the function to be executed, which must be a closure
    /// that implements the FnOnce() and Send traits.
    /// 
    /// # Panics
    /// 
    /// The `execute` function will panic if sender could not be unwrapped
    /// or if the message could not be send.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap_or_else(|| {
            panic!("Could not unwrap sender channel, got an error.");
        }).send(job).unwrap_or_else(|job| {
            panic!("Could not send message {job}");
        });
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }

            
        });

        Worker { 
            id: id,
            thread: Some(thread),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn some_operation(x: i32) -> i32 {
        2 + x
    }

    fn other_operation(x: i32, y: i32) -> bool {
        x == y
    }

    #[test]
    #[should_panic]
    fn threadpool_size_cannot_be_zero() {
        let _pool = ThreadPool::new(0);
    }

    #[test]
    fn two_threads_execute_operation() {
        let pool = ThreadPool::new(2);
        pool.execute(|| {
            let result = some_operation(5);
            assert_eq!(7, result);
        });
        pool.execute(|| {
            assert!(other_operation(5, 5));
            assert!(!other_operation(1, 2));
        });
    }
}