pub mod resolver;

use crate::models::Connection;
use crate::api::object::Object;
use crate::api::killmail::KillMail;

use std::sync::Mutex;
use crossbeam::sync::WaitGroup;


#[derive(Debug, PartialEq)]
pub enum Command{
    Quit,
    Wait(u64),
}

#[derive(Debug, PartialEq)]
pub enum Message{
    Killmail(KillMail),
    Object(Object),
    CheckObject(i32),
    Resolve((i32, bool)),
}

pub type Queue = crossbeam_queue::SegQueue<Message>;

type Commands = Channel<Command>;
type Unresolved = Channel<Message>;

pub struct AppContext {
    pub connection: Mutex<Connection>,
    pub server: String,
    pub client: String,
    pub timeout: u64,
    pub commands: Commands,
    pub saver_queue: Queue,
    pub unresolved: Unresolved,
}
impl AppContext {

    pub fn new<S: Into<String>>(connection: Connection, address: S, client: S, timeout: u64) -> Self {
        Self {
            connection: Mutex::new(connection),
            server: address.into(),
            client: client.into(),
            timeout: timeout,
            commands: Commands::new(),
            saver_queue: Queue::new(),
            unresolved: Unresolved::new(),
        }
    }
}

pub struct Channel<T>{
    queue: crossbeam_queue::SegQueue<T>,
    wg: WaitGroup,
}
impl Channel<Command> {
    pub fn new() -> Self {
        Self{
            queue: crossbeam_queue::SegQueue::new(),
            wg: WaitGroup::new(),
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
    pub fn new() -> Self {
        Self{
            queue: crossbeam_queue::SegQueue::new(),
            wg: WaitGroup::new(),
        }
    }

    pub fn push(&self, msg: Message) {
        self.queue.push(msg);
        // if let Ok(unparker) = self.unparker.try_lock() {
        //     unparker.unpark();
        //     info!("unpark()");
        // } else {
        //     warn!("failed to acquire mutex for unpark");
        // }
    }
    pub fn pop(&self) -> Option<Message> {
        // if 0 == self.queue.len() {
        //     info!("before park()");
        //     if let Ok(parker) = self.parker.try_lock() {
        //         parker.park();
        //         info!("park()");
        //     } else {
        //         warn!("failed to acquire mutex for park");
        //     }
        // }
        self.queue.pop().ok()
    }
    pub fn len(&self) -> usize {
        self.queue.len()
    }
}