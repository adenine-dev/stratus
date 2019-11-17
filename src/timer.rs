use std::time::{ Duration, SystemTime };

/// a simple scope based timer, when it is constructed the timer starts, 
/// when it goes out of scope/is dropped the timer stops and it prints
pub struct Timer {
    start_time: std::time::SystemTime,
    label: String
}

impl Timer {
    pub fn with_label(label: &str) -> Self {
        Timer {
            start_time: SystemTime::now(),
            label: label.to_string()
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
            start_time: SystemTime::now(),
            label: "timer".to_string()
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        println!("{} took {:?}ms to run", self.label, self.start_time.elapsed().expect("timer was inaccessble"));
    }
}