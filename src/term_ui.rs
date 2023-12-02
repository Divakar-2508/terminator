use std::{fs::File, io::{Write, Read, Seek}, str::FromStr, usize};

use sysinfo::{Pid, Process, ProcessExt, System, SystemExt};

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

    println!("3. quit, q - quit the current mode\n");

    println!(
        "4. show, shows the current list in terminate list"
    );

    println!(
        "5. del <index>, delete the process from the terminate list
Example:
    - To delete a single process from list with index 2: del 2
    - To delte multiple process with index 1, 3, 5: del 1 3 5"
    );
    println!()
}

fn show(file: &mut File,mode: Option<i32>) -> Option<Vec<String>> {
    let mut process_names = String::new();
    file.seek(std::io::SeekFrom::Start(0)).unwrap();
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

    if let Some(_) = mode {
        return Some(names);
    }

    for (index, name) in names.iter().enumerate() {
        println!("{:0>2}. {}", index+1, name);
    }
    println!();

    None
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

fn del(del_list: &mut Vec<usize>,file: &mut File) {

    let mut process_names = match show(file, Some(1)) {
        None => { return; }
        Some(value) => value,
    };

    del_list.sort();
    del_list.reverse();

    let mut count = 0;

    for &to_be_deleted in del_list.iter() {
        if to_be_deleted < del_list.len() {
            process_names.remove(to_be_deleted);
            count += 1;
        } else {
            println!("Can't delete {}, no element in index!", to_be_deleted);
        }
    }

    match file.set_len(0) {
        Ok(_) => {}
        Err(e) => {
            println!("Can't delete the content\nError: {}", e);
        }
    }
    
    for process in process_names {
        writeln!(file, "{}", process).unwrap();
    }

    println!("\nDeleted {} process from list, use 'show' to get the current list\n", count);
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
                show(file, None);
            }
            "del" => {
                
                let mut del_ids: Vec<usize> = command_real[1..]
                    .into_iter()
                    .filter_map(|x| {
                        let value = x.parse::<usize>();
                        if let Ok(value) = value {
                            Some(value + 1)
                        } else {
                            None
                        }
                    }
                    )
                    .collect();

                if del_ids.is_empty() {
                    println!("Please specify the indexes");
                    return;
                }

                del(&mut del_ids, file)
            }
            _ => println!("Invalid command\n"),
        }
    }
}
