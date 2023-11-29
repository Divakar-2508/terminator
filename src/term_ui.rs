use std::{fs::File, io::{Write, Read}, str::FromStr, usize};

use sysinfo::{Pid, Process, ProcessExt, System, SystemExt};

fn add(sys: &System, ids: Vec<Pid>, file: &mut File) {
    // Open the file "process_names.txt" in append mode
    // Iterate over the process IDs
    for id in ids {
        // Get the process with the given ID
        let process = sys.process(id);
        if let Some(process) = process {
            // Get the process name
            let name = process.name();
            // Write the process name to the file
            writeln!(file, "{}", name).unwrap();
        } else {
            println!("No process with the ID {} found", id);
            continue;
        }
    }
}

fn help() {
    println!(
        "1. list <filters> - List running processes
Example:
    - To list all processes: list
    - To list processes starting with 's': list s
    - To list processes starting with 's' and 'wor': list s wor\n"
    );

    println!(
        "2. add <Pid...> - Add processes to Terminator      
Example:
    - To add a single process with PID 1234: add 1234
    - To add multiple processes with PIDs 5678, 9012, and 3456: add 5678 9012 3456
        "
    );

    println!("3. quit, q - quit the current mode");
}

fn show(file: &mut File) -> Option<Vec<String>> {
    let mut process_names = String::new();
    match file.read_to_string(&mut process_names) {
        Ok(_) => {},
        Err(e) => {
            println!("Can't read the file\nError: {}", e);
            return None;
        }
    }
    if process_names.is_empty() {
        println!("File is Empty, add files with 'list' and 'add'");
        return None;
    }

    let names: Vec<String> = process_names.lines()
        .map(|x| x.to_string())
        .collect();

    for (index, name) in names.iter().enumerate() {
        println!("{:0>}. {}", index, name);
    }
    
    return Some(names);
}

fn list(sys: &System, filters: Vec<String>) {
    let processes = sys.processes();

    if filters.is_empty() {
        for (pid, process) in processes {
            println!("{} : {}", pid, process.name());
        }
    } else {
        for filter in &filters {
            let match_process = processes
                .into_iter()
                .filter(|x| x.1.name().to_lowercase().starts_with(&filter.to_lowercase()))
                .collect::<Vec<(&Pid, &Process)>>();
            for (pid, process) in match_process {
                println!("{} : {}", pid, process.name());
            }
        }
    }
    println!("You can use 'add <pid>' (the number) to add the process to terminate");
}

fn del(del_list: &mut Vec<usize>,file: &mut File) {

    let mut process_names = match show(file) {
        None => { return; }
        Some(value) => value,
    };

    del_list.sort();
    del_list.reverse();

    for to_be_deleted in del_list {
        process_names.remove(*to_be_deleted);
    }

    // file
}

pub fn run(sys: &System, file: &mut File) {
    let mut command = String::new();
    println!("Terminator UI Mode Started!");
    println!("type 'help' to get going\n");
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        command.clear();
        std::io::stdin().read_line(&mut command).unwrap();

        let command_real = command
            .split_ascii_whitespace()
            .map(|x| x.to_string().to_ascii_lowercase())
            .collect::<Vec<String>>();

        match command_real[0].to_lowercase().trim(){
            "help" => help(),
            "list" => {
                list(&sys, command_real[1..].to_vec());
            }
            "q" | "quit" => {
                println!("\nTerminating UI Mode");
                break;
            }
            "add" => {
                let pids = command_real[1..]
                    .into_iter()
                    .map(|x| Pid::from_str(&x).unwrap())
                    .collect();
                add(&sys, pids, file);
            }
            "show" => {
                show(file);
            }
            "del" => {
                let mut del_id: Vec<usize> = command_real[1..]
                    .into_iter()
                    .filter_map(|x| Some(x.parse::<usize>().unwrap()))
                    .collect();
                if del_id.is_empty() {
                    println!("Please specify the indexes");
                    return;
                }
                del(&mut del_id, file)
            }
            _ => println!("Invalid command\n"),
        }
    }
}
