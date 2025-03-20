use std::{
    error::Error,
    fmt::Display,
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
    thread::{self, JoinHandle},
};

struct Worker {
    _id: usize,
    _thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let _thread = thread::spawn(move || {
            loop {
                let job = receiver
                    .lock()
                    .expect("Thread was poisoned, could not acquire lock...")
                    .recv()
                    .expect("There was something wrong with the job...");

                println!("Got a job in thread {id}");

                job();
            }
        });
        Self { _id: id, _thread }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
pub struct PoolCreationError(String);

impl Display for PoolCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)?;
        Ok(())
    }
}

impl Error for PoolCreationError {}
pub struct ThreadPool {
    _workers: Vec<Worker>,
    sender: Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(num_of_threads: usize) -> Self {
        assert!(num_of_threads > 0);

        Self::safely_build_self(num_of_threads)
    }

    pub fn build(num_of_threads: usize) -> Result<Self, PoolCreationError> {
        if num_of_threads == 0 {
            return Err(PoolCreationError(String::from(
                "Can't create a thread pool with 0 threads",
            )));
        }

        Ok(Self::safely_build_self(num_of_threads))
    }

    fn safely_build_self(num_of_threads: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let _workers = (0..num_of_threads)
            .into_iter()
            .map(|i| Worker::new(i, Arc::clone(&receiver)))
            .collect();
        Self { _workers, sender }
    }

    pub fn execute<F>(&mut self, cb: F)
    where
        F: FnOnce() -> () + Send + 'static,
    {
        self.sender.send(Box::new(cb)).unwrap()
    }
}
