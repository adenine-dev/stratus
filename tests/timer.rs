#![feature(test)]

#[macro_use] 
extern crate stratus;

#[test]
fn takes_time() {
    let timer = stratus::Timer::default();
    std::thread::sleep(std::time::Duration::new(2, 0));
}