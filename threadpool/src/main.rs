use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::collections::VecDeque;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Arc<Mutex<VecDeque<Job>>>,
    cvar: Arc<Condvar>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        assert!(size > 0);
        let sender = Arc::new(Mutex::new(VecDeque::new()));
        let cvar = Arc::new(Condvar::new());

        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&sender), Arc::clone(&cvar)));
        }

        ThreadPool { workers, sender, cvar }
    }

    fn execute<F>(&self, job: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(job);

        let mut queue = self.sender.lock().unwrap();
        queue.push_back(job);
        drop(queue);
        self.cvar.notify_one();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // First signal to all the workers that no more jobs will be coming.
        {
            let mut queue = self.sender.lock().unwrap();
            for _ in &self.workers {
                // Placing `None` on the queue will signal a worker to exit.
                queue.push_back(Box::new(|| {}));
            }
        }
        // Wake up all the workers so they will notice the `None` and exit.
        self.cvar.notify_all();

        // Take ownership of the workers so we can join their threads.
        // This avoids the issue of multiple mutable borrows.
        while let Some(worker) = self.workers.pop() {
            worker.terminate();
        }
    }
}


struct Worker {
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(receiver: Arc<Mutex<VecDeque<Job>>>, cvar: Arc<Condvar>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job;
            {
                let mut queue = receiver.lock().unwrap();
                while queue.is_empty() {
                    queue = cvar.wait(queue).unwrap();
                }
                job = queue.pop_front();
            }

            if let Some(job) = job {
                job();
            }
        });

        Worker {
            thread: Some(thread),
        }
    }

    fn terminate(mut self) {
        // Take the thread out of the Option so we can use it.
        if let Some(thread) = self.thread.take() {
            // We ignore any errors here, as we can't recover in a Drop implementation.
            let _ = thread.join();
        }
    }
}

fn main() {
    let pool = ThreadPool::new(4);

    for _ in 0..8 {
        pool.execute(|| {
            // your code here
            println!("Job is running");
            std::thread::sleep(std::time::Duration::from_millis(10));
        });
    }
    drop(pool);
}