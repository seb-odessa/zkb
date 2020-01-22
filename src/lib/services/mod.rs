pub mod server;
pub mod monitor;
pub mod resolver;
pub mod database;

use crate::api;
use crate::models;
use std::sync::{Arc, Mutex, Condvar};
use uuid::adapter::Simple as Uid;

use chrono::{DateTime, Utc};
use actix_web::web;

pub type Context = web::Data<AppContext>;

pub use models::system::SystemFilter;

#[derive(Debug, PartialEq)]
pub enum Command{
    Quit,
}

#[derive(Debug, PartialEq)]
pub enum Api{
    Object(i32),
    System(i32),
    Stargate(i32),
    Constellation(i32),
}

#[derive(Debug, PartialEq)]
pub enum Model{
    Object(api::object::Object),
    System(api::system::System),
    Stargate(api::stargate::Stargate),
    Constellation(api::constellation::Constellation),
    Killmail(api::killmail::Killmail),
    Observatory(i32),
}

#[derive(Debug, PartialEq)]
pub enum Area{
    System(i32),
    Constellation(i32),
    Region(i32),
}

#[derive(Debug, PartialEq)]
pub enum Actor{
    Character(i32),
    Corporation(i32),
    Alliance(i32),
    Faction(i32),
}


#[derive(Debug, PartialEq)]
pub enum Category{
    Object(i32),
    Killmail(i32),
    Victim(i32),
    Attackers(i32),
    Items(i32),
    ObservatoryPath(i32),
    System(i32),
    Region(i32),
    Constellation(i32),
    Systems((Area, SystemFilter)),
    Constellations(Area),
    Stargate(i32),
    History((Area, i32)),
    Wins((Actor, i32)),
    Losses((Actor, i32)),
    HistoryCount((Area, i32)),
    ObjectDesc((String, String)),
    Neighbors(Area),
}

#[derive(Debug, PartialEq)]
pub enum Report{
    Killmail(models::killmail::KillmailNamed),
    Victim(models::victim::VictimNamed),
    Attackers(Vec<models::attacker::AttackerNamed>),
    Items(Vec<models::item::ItemNamed>),
    History(Vec<models::killmail::KillmailNamed>),
    HistoryCount(i32),
    System(models::system::SystemNamed),
    Region(models::region::RegionNamed),
    Constellation(models::constellation::ConstellationNamed),
    Systems(Vec<models::system::SystemNamed>),
    SystemNeighbors(Vec<models::system::SystemNeighbors>),
    ConstellationNeighbors(Vec<models::constellation::ConstellationNeighbors>),
    RegionNeighbors(Vec<models::region::RegionNeighbors>),
    Constellations(Vec<models::constellation::ConstellationNamed>),
    ObservatoryPath(Vec<models::system::ObservatoryPath>),
    Object(models::object::Object),
    Id(i32),
    NotFoundId(i32),
    NotFoundName(String),
    NotUniqName(String),
    QueryFailed(String),
}

#[derive(Debug, PartialEq)]
pub enum Message{
    Ping,
    Receive(Api),
    Save(Model),
    Delete(Model),
    Check(Category),
    Report((Uid, Report)),
    Find((Uid, Category)),
}

type Commands = Channel<Command>;
type Queue = Channel<Message>;
type Guard = Arc<(Mutex<bool>, Condvar)>;

pub struct AppContext {
    pub server: String,
    pub client: String,
    pub timeout: u64,
    pub allowed: Option<DateTime<Utc>>,
    pub commands: Commands,
    pub database: Queue,
    pub resolver: Queue,
    pub responses: Queue,
}
impl AppContext {

    pub fn new<S: Into<String>>(address: S, client: S, timeout: u64, allowed: Option<DateTime<Utc>>) -> Self {
        Self {
            server: address.into(),
            client: client.into(),
            timeout: timeout,
            allowed: allowed,
            commands: Commands::new(Arc::new((Mutex::new(false), Condvar::new()))),
            database: Queue::new(Arc::new((Mutex::new(false), Condvar::new()))),
            resolver: Queue::new(Arc::new((Mutex::new(false), Condvar::new()))),
            responses:Queue::new(Arc::new((Mutex::new(false), Condvar::new()))),
        }
    }

    pub fn get_root(&self) ->String {
        format!("http://{}/navigator", &self.server)
    }

    pub fn get_api_link<S1: Into<String>, S2: Into<String>>(&self, class: S1, name: S2) -> String {
        format!(r#"<a href="{root}/api/{class}/{name}">{name}</a>"#,
            root = self.get_root(),
            class = class.into(),
            name = name.into())
    }

    pub fn get_api_href<S1: Into<String>, S2: Into<String>>(&self, class: S1, id: i32, name: S2) -> String {
        format!(r#"<a href="{root}/api/{class}/{id}">{name}</a>"#,
            root = self.get_root(),
            class = class.into(),
            id = id,
            name = name.into())
    }

    pub fn get_zkb_href<S1: Into<String>, S2: Into<String>>(&self, class: S1, id: i32, name: S2) -> String {
        format!(r#"<a href="https://zkillboard.com/{class}/{id}/">{name}</a>"#,
            class = class.into(),
            id = id,
            name = name.into())
    }

    pub fn get_dotlan_href<S1: Into<String>, S2: Into<String>, S3: Into<String>>(&self, region: S1, system: S2, name: S3) -> String {
        let region = region.into().replace(" ", "_");
        let system = system.into().replace(" ", "_");
        let name = name.into();
        if system.is_empty() {
            format!(r#"<a href="http://evemaps.dotlan.net/map/{}">{}</a>"#, region, name)
        } else {
            format!(r#"<a href="http://evemaps.dotlan.net/map/{}/{}/">{}</a>"#, region, system, name)
        }
    }

    pub fn get_evewho_href<S1: Into<String>, S2: Into<String>>(&self, class: S1, id: i32, name: S2) -> String {
        format!(r#"<a href="https://evewho.com/{class}/{id}/">{name}</a>"#,
            class = class.into(),
            id = id,
            name = name.into())
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