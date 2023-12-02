#![allow(non_snake_case)]

pub mod term_ui;

use std::{env, fs::{self, OpenOptions}};

use sysinfo::{System, SystemExt, ProcessExt};

fn read() -> Vec<String> {
    if let Ok(process) = fs::read_to_string("process_names.txt") {
        let process_names = process.lines().map(|x| x.to_string()).collect::<Vec<_>>();
        process_names
    } else {
        eprintln!("Can't read process_names.txt");
        println!("Please use 'terminator ui' and add the processes to terminate");
        std::process::exit(1);
    }
    
}

fn terminate(sys: &mut System) {
    let process_names = read();
    if process_names.is_empty() {
        println!("The process list is empty!");
        println!("use 'terminator ui' to add processes");
        return;
    }

    sys.refresh_all();

    sys.processes()
        .values()
        .filter(|proc| process_names.contains(&proc.name().to_string()))
        .filter(|proc| proc.kill_with(sysinfo::Signal::Kill) != Some(true))
        .filter(|proc| proc.kill_with(sysinfo::Signal::Term) != Some(true))
        .for_each(|proc|{
            eprintln!(
                "Can't kill this guy \"{}\", pid : {}",
                proc.name(),
                proc.pid(),
            )
        });
    println!("Done");
}

fn main(){
    let arg: Vec<String> = env::args().skip(1).collect();
    let mut sys = System::new_all();

    let file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("process.txt");

    let mut file = match file {
        Ok(file) => file,
        Err(_) => {
            println!("Seems Like Your First Time, Run 'terminator ui'");
            return;
        }
    };

    if arg.len() > 0 {
        term_ui::run(&sys, &mut file);
        return;
    }
    terminate(&mut sys);
}