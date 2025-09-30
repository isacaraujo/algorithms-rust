use std::{error::Error, panic, sync::{mpsc::{self}, Arc, Mutex}, thread::{self, JoinHandle}, time::Duration};

use log::{debug, error, info, trace};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    handler: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let handler = thread::spawn(move || {
            loop {
                // It forces mutex to lock the receiver just while
                // receiving an job
                let job = {
                    let lock = receiver.lock().unwrap();
                    lock.recv()
                };

                match job {
                    Ok(job) => {
                        trace!(target: "HANDLER", "Worker {id} received a job");
                        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                            job();
                        }));

                        if let Err(e) = result {
                            if let Some(err_msg) = e.downcast_ref::<&str>() {
                                error!(target: "HANDLER", "Worker {id} job panicked: {err_msg}");
                            } else {
                                error!(target: "HANDLER", "Worker {id} job panicked: {:?}", e);
                            }
                        }
                    },
                    Err(err) => {
                        trace!(target: "HANDLER", "Worker {} received an error: {}", id, err);
                        break;
                    }
                }
            }

        });
        Worker { id, handler }
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..size {
            workers.insert(i, Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    fn execute<F>(&self, f: F) -> Result<(), Box<dyn Error>>
    where
        F: FnOnce() + Send + 'static
    {
        match &self.sender {
            Some(sender) => {
                let job = Box::new(f);
                sender.send(job)?;
                Ok(())
            },
            None => Err("[EXECUTE] Sender is shutting down".into()),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        trace!(target: "DROP", "ThreadPool is shutting down ...");

        while let Some(worker) = self.workers.pop() {
            trace!(target: "DROP", "Shutting down worker {}", worker.id);
            if let Err(payload) =  worker.handler.join() {
                if let Some(s) = payload.downcast_ref::<&str>() {
                    debug!(target: "DROP", "Failed to join Worker({}).handler. Err: {}", worker.id, s);
                } else {
                    debug!(target: "DROP", "Failed to join Worker({}).handler", worker.id);
                }
            }
        }

        trace!(target: "DROP", "All workers is done");
    }
}

fn main() {
    env_logger::init();

    info!(target: "MAIN", "ThreadPool program is initialized");

    {
        let pool = ThreadPool::new(4);

        for (i, word) in ["lorem", "ipsum", "is", "a", "dummy"].iter().enumerate() {
            let result = pool.execute(move || {
                info!(target: "MAIN", "Processing word {i}: {word}");
                thread::sleep(Duration::from_millis(500));
            });

            match result {
                Ok(_) => {},
                Err(e) => {
                    error!(target: "PROG", "Error: {:?}", e);
                },
            };
        }

        info!(target: "MAIN", "Waiting for threads to be completed ...");
    }

    info!(target: "MAIN", "Goodbye");
}
