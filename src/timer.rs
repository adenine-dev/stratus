use std::time::{ Duration, SystemTime };

struct Timer {
    start_time: std::time::SystemTime
}

impl Timer {
    
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
            start_time: SystemTime::now()
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {

    }
}