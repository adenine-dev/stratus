#![feature(test)]

#[macro_use]
extern crate stratus;

#[test]
fn takes_time() {
    let timer = stratus::Timer::default();
    std::thread::sleep(std::time::Duration::new(1, 0));
}

#[test]
fn use_macro() {
    time_scope!(std::thread::sleep(std::time::Duration::new(2, 0)));
}

#[test]
fn with_label() {
    time_scope!(
        "timer with a label",
        std::thread::sleep(std::time::Duration::new(3, 0))
    );
}

#[test]
fn timed_function() {
    std::thread::sleep(std::time::Duration::new(4, 0))
}
