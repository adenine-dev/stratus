use std::time::{ SystemTime };

/// a simple scope based timer, when it is constructed the timer starts,
/// when it goes out of scope/is dropped the timer stops and it prints
pub struct Timer {
    start_time: SystemTime,
    label: String,
}

impl Timer {
    /// creates a new timer without a label
    pub fn new() -> Self {
        Self::default()
    }

    /// creates a new timer with a label this will be printed along with the data
    pub fn new_with_label(label: &str) -> Self {
        Timer {
            start_time: SystemTime::now(),
            label: label.to_string(),
        }
    }

    /// prints the amount of time that has elapsed since the timer's creation
    pub fn elapsed(&self) {
        println!(
            "{} has taken {:?} to run",
            self.label,
            self.start_time.elapsed().expect("timer was inaccessble")
        );
    }
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
            start_time: SystemTime::now(),
            label: "timer".to_string(),
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        println!(
            "{} took {:?} to run",
            self.label,
            self.start_time.elapsed().expect("timer was inaccessble")
        );
    }
}

/// Times the scope passed to the macro, either with or without a label. When everything 
/// passed to this scope has executed the amount of time it took will be printed
/// ## Example:
/// ```no-run
/// time_scope!(println!("hello!"));
/// time_scope!({
///     for i in 0..1000 {
///         println!("{}", i);
///     }
/// });
/// time_scope!("print something", println!("hello!"));
/// ```
#[macro_export]
macro_rules! time_scope {
    ($x:expr) => {
        let timer = stratus::Timer::default();
        $x;
    };

    ($label:expr, $x:expr) => {
        let timer = stratus::Timer::new_with_label($label);
        $x;
    };
}