use std::ops::Deref;


struct MyBox<T> (T);

impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl <T> MyBox<T> {
    fn new(x : T) -> MyBox<T> {
        return MyBox(x);
    }
}

use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

/*
pub struct Worker {
    id : usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Ok(job) => {
                        println!("Woker {id} got a job, executing ... ");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected, shutting down");
                        break;
                    }
                }
            }
        });
        return Worker{
            id,
            thread: Some(thread),
        }
    }
}

pub type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

impl ThreadPool {
    pub fn new (size : usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        return ThreadPool{
            workers,
            sender: Some(sender)
        };
    }

    pub fn execute<F>(&self, f : F)
    where
    F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers{
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}
*/
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_deref(){
        let x = 1;
        let y = MyBox::new(x);
        assert_eq!(x, *y);
    }

}