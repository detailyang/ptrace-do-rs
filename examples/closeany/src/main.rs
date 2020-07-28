extern crate getopts;
extern crate ptrace_do_rs;

use ptrace_do_rs::*;
use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} -p pid [fd]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("p", "", "set the process pid to close", "PID");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let fds =  matches.free.iter().map(|fd|{
        fd.parse::<i32>().expect("expect input valid fd(integer)")
    }).collect();

    let pid = matches.opt_str("p").map(|e| e.parse::<i32>().expect("expect valid pid")).expect("must input pid");

    do_close(pid, fds);
}

fn do_close(pid: i32, fds: Vec<i32>) {
    println!("try close pid({}) fd with {:?}", pid, fds);
    unsafe {
        let target = ptrace_do_init(pid);
        if target.is_null() {
            return;
        }
        fds.into_iter().for_each(|fd| {
            ptrace_do_syscall(target, __NR_close.into(), fd as u64, 0, 0, 0, 0, 0);
        });
        ptrace_do_cleanup(target);
    }
}