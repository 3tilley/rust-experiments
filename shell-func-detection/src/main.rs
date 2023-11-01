use std::fmt::format;
use std::fs;
use std::io::Write;
use std::path::Path;
use sysinfo::{Pid, Process, ProcessExt, System, SystemExt};

fn main() {
    println!("Command started");
    let args = std::env::args();
    let str_args = args.map(|a| a.to_string()).collect::<Vec<_>>().join(" ");

    let out_vars = std::env::vars().map(|(k,v)| format!("{}={}", k, v)).collect::<Vec<_>>();

    let f = fs::File::create("env.txt").unwrap();
    fs::write("env.txt", out_vars.join("\n")).unwrap();

    println!("{}", str_args);

    let s = System::new_all();
    if let Some(process) = s.process(Pid::from(std::process::id() as usize)) {
        println!("Current process: {:?}", process.cmd());

        show_args(&s, process.pid(), 1);

    }
}

fn show_args(s: &System, pid: Pid, level: usize) {
    let proc = s.process(pid).unwrap();
    let p_proc = s.process(proc.parent().unwrap()).unwrap();
    let str_p_args = p_proc.cmd().join(" ");
    println!("Level {}: process args: {}", level, str_p_args);
    if let Some(p_proc) = s.process(p_proc.pid()).unwrap().parent() {
        show_args(s, p_proc, level + 1)
    }
}
