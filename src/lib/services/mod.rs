pub mod monitor;
pub mod resolver;
pub mod saver;

use crate::models::Connection;
use crate::api::object::Object;
use crate::api::killmail::KillMail;

use std::sync::{Arc, Mutex, Condvar};


#[derive(Debug, PartialEq)]
pub enum Command{
    Quit,
    Wait(u64),
}

#[derive(Debug, PartialEq)]
pub enum Message{
    Ping,
    Killmail(KillMail),
    Object(Object),
    CheckObject(i32),
    Resolve((i32, bool)),
}


type Commands = Channel<Command>;
type Queue = Channel<Message>;
type Guard = Arc<(Mutex<bool>, Condvar)>;

pub struct AppContext {
    pub connection: Mutex<Connection>,
    pub server: String,
    pub client: String,
    pub timeout: u64,
    pub commands: Commands,
    pub saver: Queue,
    pub unresolved: Queue,
}
impl AppContext {

    pub fn new<S: Into<String>>(connection: Connection, address: S, client: S, timeout: u64) -> Self {
        Self {
            connection: Mutex::new(connection),
            server: address.into(),
            client: client.into(),
            timeout: timeout,
            commands: Commands::new(Arc::new((Mutex::new(false), Condvar::new()))),
            saver: Queue::new(Arc::new((Mutex::new(false), Condvar::new()))),
            unresolved: Queue::new(Arc::new((Mutex::new(false), Condvar::new()))),
        }
    }
}

pub struct Channel<T>{
    queue: crossbeam_queue::SegQueue<T>,
    guard: Arc<(Mutex<bool>, Condvar)>,
}
impl Channel<Command> {
    pub fn new(guard: Guard) -> Self {
        Self{
            queue: crossbeam_queue::SegQueue::new(),
            guard: guard,
        }
    }
    
    pub fn push(&self, msg: Command) {
        self.queue.push(msg);
    }

    pub fn pop(&self) -> Option<Command> {    
        self.queue.pop().ok()
    }
    
    pub fn len(&self) -> usize {
        self.queue.len()
    }
}

impl Channel<Message> {
    pub fn new(guard: Guard) -> Self {
        Self{
            queue: crossbeam_queue::SegQueue::new(),
            guard: guard,
        }
    }

    fn reset(&self, value: bool) {
        let (lock, condition) = &*self.guard;
        let mut ready = lock.lock().unwrap();
        *ready = value;
        if value {
            condition.notify_all();
        }
    }

    fn wait_notification(&self) {
        let (lock, var) = &*self.guard;
        let mut ready = lock.lock().unwrap();
        while !*ready {
            ready = var.wait(ready).unwrap();
        }
    }

    pub fn push(&self, msg: Message) {
        self.queue.push(msg);
        self.reset(true);
    }
    
    pub fn pop(&self) -> Option<Message> {
        self.wait_notification();        
        let result = self.queue.pop().ok();
        if 0 == self.len() {
            self.reset(false);
        }
        return result;
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }
}