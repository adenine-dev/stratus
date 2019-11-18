extern crate once_cell;
use once_cell::sync::OnceCell;
use std::sync::Mutex;

use std::time::{ SystemTime };

/// A Profile is profiling data for a given bit of time recorded to the instrumentor.
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
    /// creates a new profile, you probably want to use #[profiled] on a given function.
    /// Otherwise category and name are values shown in the tracer.
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

    /// "stops" the profile recording, this actually just records the end time.
    pub fn stop(&mut self) {
        self.end_time = SystemTime::now();
    }

    /// converts the given profile into json.
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
            //TODO: make this not awful because as is it is kinda bad
            tid = (format!("{:?}", self.thread)[9..]).to_string()
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

/// The instrumentor is the overarching class that handles the creation of 
/// the json file that is fed into chrome://tracing (https://www.chromium.org/developers/how-tos/trace-event-profiling-tool)
/// this is used as a singleton and you almost certainly don't want to make an instance of it.
pub struct Instrumentor {
    active_session: bool,
    time_data: Vec<Profile>,
    name: String,
    profiler_start: SystemTime
}

impl Instrumentor {
    /// initializes the instrumentor
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

    /// start an intrumentation session. `name` will be the name of the json file. 
    pub fn start_session(name: &str) {
        let mut instrumentor = INSTRUMENTOR.get().expect("failed to get instrumentor").try_lock().unwrap();
        instrumentor.active_session = true;
        instrumentor.name = name.to_string();
        instrumentor.time_data.clear();
        instrumentor.profiler_start = SystemTime::now();
    }

    /// pushes a profile to the time data, this will be reflected in the resulting json
    pub fn push_profile(data: Profile) {
        INSTRUMENTOR.get().expect("failed to get instrumentor").try_lock().unwrap().time_data.push(data);
    }

    /// this ends a session and writes the json file to the disk.
    pub fn end_session() {
        let instrumentor = INSTRUMENTOR.get().expect("failed to get instrumentor").try_lock().unwrap();

        let mut json = "{\"traceEvents\": [".to_string();

        for (i, data) in (&instrumentor.time_data).into_iter().enumerate() {
            json.push_str(&data.to_json(instrumentor.profiler_start));
            if i != instrumentor.time_data.len() - 1 { json.push_str(","); }
        }

        json.push_str("] }");
        std::fs::write([ &instrumentor.name, ".json" ].concat(), json).expect("biggest oof");
    }
}