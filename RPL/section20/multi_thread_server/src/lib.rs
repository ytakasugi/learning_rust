use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

trait FnBox {
    // self: Box<Self>を取ってselfの所有権を奪い、Box<T>から値をムーブする
    fn call_box(self: Box<Self>);
}

// FnOnce()トレイトを実装する任意の型Fに対してFnBoxトレイトを実装
// あらゆるFnOnce()クロージャがcall_boxメソッドを使用できる
impl<F: FnOnce()> FnBox for F {
    // (*self)()を使用してBox<T>からクロージャをムーブし、クロージャを呼び出す
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    /// 新しいThreadPoolを生成する。
    ///
    /// sizeがプールのスレッド数です。
    ///
    /// # パニック
    ///
    /// sizeが0なら、`new`関数はパニックします。
    ///
    // id番号を取り、idと空のクロージャで大量生産されるスレッドを保持するWorkerインスタンスを返す
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // 新しいチャンネルを作成
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        // size個の要素を保持できる新しい空のベクタを生成
        let mut workers = Vec::with_capacity(size);
        // ThreadPool::newでforループカウンタを使用してidを生成し、そのidで新しいWorkerを生成し、ベクタにWorkerを格納
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

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// idとJoinHandle<()>を保持するWorker構造体
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}


impl Worker {
    // 複数のスレッドで所有権を共有しつつ、 スレッドに値を可変化させるためには、Arc<Mutex<T>>を使用
    // Arc型で複数のWorkerにreceiverを共有させ、Mutexにより、1度にreceiverから1つの仕事をたった1つのWorkerが受け取ることを保証する
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->
        Worker {
        // 空のクロージャを使って新しいスレッドを立ち上げ、生成されるJoinHandle<()>インスタンスを格納
        let thread = thread::spawn(move ||{
            loop {
                // receiverに対して`lock()`を呼び出し、mutexを取得。失敗する可能性があるため`unwrap()`を使用
                // recvを呼び出してチャンネルからJobを受け取る
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}