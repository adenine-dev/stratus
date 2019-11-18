#![feature(test)]

#[macro_use]
extern crate stratus;

// hack __setup will run first, zz_shutdown will run last, this is temporary and will only 
// be used long enough to have a test runner
#[test]
fn __setup() {
    stratus::Instrumentor::init_instrumentor();
    stratus::Instrumentor::start_session("test instrumentaion session");
}

#[test]
fn zz_shutdown() {
    stratus::Instrumentor::end_session();
}

#[test]
#[profiled]
fn takes_time() {
    let _timer = stratus::Timer::default();
    std::thread::sleep(std::time::Duration::from_millis(100));
}

#[test]
#[profiled]
fn use_macro() {
    time_scope!(std::thread::sleep(std::time::Duration::from_millis(100)));
}

#[test]
#[timed]
#[profiled]
fn use_proc_macro() {
    std::thread::sleep(std::time::Duration::from_millis(200));
}

#[test]
#[profiled]
fn with_label() {
    time_scope!(
        "timer with a label",
        std::thread::sleep(std::time::Duration::from_millis(300))
    );
}

#[test]
#[profiled]
fn timed_function() {
    std::thread::sleep(std::time::Duration::from_millis(400))
}
