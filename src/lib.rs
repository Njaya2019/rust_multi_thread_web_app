use std::{sync::{mpsc, Arc, Mutex}, thread};
pub struct ThreadPool{
    workers: Vec<Woker>,
    sender: Option<mpsc::Sender<Job>>
}
type  Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver =Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size{
            // Create some threads and store them in the vector
            workers.push(Woker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
       let job = Box::new(f);
       self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl  Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for woker in &mut self.workers{
            println!("Shutting down worker {}", woker.id);

            if let Some(thread)  = woker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
    
}

struct Woker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Woker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Woker{
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job, executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected, shutting down.");
                    break;
                }
                
            }

        });

        Woker {id, thread: Some(thread)}
    }
    
}