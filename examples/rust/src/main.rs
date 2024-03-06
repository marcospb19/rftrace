extern crate rftrace as _;
use rftrace_frontend as rftrace;

fn main() {
    let events = rftrace::init(2000, false);
    rftrace::enable();
    println!("Hello, world!");
    test1();
    rftrace::dump_full_uftrace(events, "tracedir", "test", true).expect("");
}

fn test1() {
    println!("test1");
    test2();
}

fn test2() {
    println!("test2");
    test3();
}

fn test3() {
    println!("test3");
}
