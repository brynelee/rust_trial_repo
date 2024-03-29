use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

//type Job = Box<FnOnce() + Send + 'static>;

trait FnBox{
    fn call_box(self: Box<Self>);
}

impl <F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>){
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;



pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// 创建线程池。
    ///
    /// 线程池中线程的数量。
    ///
    /// # Panics
    ///
    /// `new` 函数在 size 为 0 时会 panic。
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        let receiver= Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }

}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {

            loop {

                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Work {} got a job", id);

                job.call_box();

            }

        });

        Worker {
            id,
            thread,
        }
    }
}