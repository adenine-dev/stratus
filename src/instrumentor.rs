extern crate once_cell;
use once_cell::sync::OnceCell;
use std::sync::Mutex;

use std::time::{ SystemTime };

#[derive(Clone)]
pub struct Profile {
    category: String,
    name: String,
    start_time: SystemTime,
    end_time: SystemTime,
    thread: std::thread::ThreadId,
    process: u32,
}

impl Profile {
    pub fn new(category: String, name: String) -> Profile {
        Profile {
            category,
            name,
            start_time: SystemTime::now(),
            end_time: SystemTime::now(),
            process: std::process::id(),
            thread: std::thread::current().id()
        }
    }

    pub fn stop(&mut self) {
        self.end_time = SystemTime::now();
    }

    pub fn to_json(&self, profiler_start: SystemTime) -> String {
        format!(
            r#"{{
                "name": "{name}",
                "cat": "{cat}",
                "ph": "X",
                "ts": {ts:?},
                "dur": {dur:?},
                "pid": {pid},
                "tid": "{tid}"
            }}"#,
            name = self.name,
            cat = self.category,
            ts = self.start_time.duration_since(profiler_start).expect("").as_millis(),
            dur = self.end_time.duration_since(self.start_time).expect("").as_millis(),
            pid = self.process,
            tid = (format!("{:?}", self.thread)[9..10]).to_string()
        )
    }
}

impl Drop for Profile {
    fn drop(&mut self) {
        if self.start_time == self.end_time {
            self.stop();
        }
        Instrumentor::push_profile(self.clone());
    }
}

static INSTRUMENTOR: OnceCell<Mutex<Instrumentor>> = OnceCell::new();

pub struct Instrumentor {
    active_session: bool,
    time_data: Vec<Profile>,
    name: String,
    profiler_start: SystemTime
}

impl Instrumentor {
    pub fn init_instrumentor() {
        INSTRUMENTOR.get_or_init(|| {
            Mutex::new(Instrumentor {
                active_session: false,
                time_data: vec!(),
                name: "".to_string(),
                profiler_start: SystemTime::now()
            })
        });
    }

    pub fn start_session(name: &str) {
        let mut instrumentor = INSTRUMENTOR.get().expect("failed to get instrumentor").try_lock().unwrap();
        instrumentor.active_session = true;
        instrumentor.name = name.to_string();
        instrumentor.time_data.clear();
        instrumentor.profiler_start = SystemTime::now();
    }

    pub fn push_profile(data: Profile) {
        INSTRUMENTOR.get().expect("failed to get instrumentor").try_lock().unwrap().time_data.push(data);
    }

    pub fn end_session() {
        let instrumentor = INSTRUMENTOR.get().expect("failed to get instrumentor").try_lock().unwrap();

        let mut json = "{\"traceEvents\": [".to_string();

        for (i, data) in (&instrumentor.time_data).into_iter().enumerate() {
            json.push_str(&data.to_json(instrumentor.profiler_start));
            if i != instrumentor.time_data.len() - 1 { json.push_str(","); }
        }

        json.push_str("] }");
        std::fs::write("data.json", json).expect("biggest oof");
    }
}