// Modules
mod graphics;
mod commands;
mod memory;

// Includes
use commands::*;
use device_query::DeviceState;
use memory::*;
use graphics::*;
use core::panic;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};


/*
    `fn main()` will handle the following:
     - Collecting path to .glg file
     - Adding Glang commands
*/
fn main() {
    // Local variables
    let path;
    let args : Vec<String> = env::args().collect();

    if args.len() != 2
    {
        panic!("ERR: No path provided\nUsage: glang.exe /path/to/code(*.glg))")
    }
    path = args[1].clone();

    // Global variables
    let mut globals = Globals
    {
        query : Query::new(),
        arg_numbers : HashMap::new(),
        commands : vec![],
        stack  : Stack::new(),
        labels  : HashMap::new(),
        cursor : 0,
        graphics : Graphics::default(),
        keyboard : DeviceState::new(),
        keys : vec![]
    };
    
    // List of commands
    // Please for the love of everything holy, fix this
    add_command(&mut globals.query, &mut globals.arg_numbers, "alive", alive,0);
    add_command(&mut globals.query, &mut globals.arg_numbers, "cursor", cursor,0);
    add_command(&mut globals.query, &mut globals.arg_numbers, "print", print,1);
    add_command(&mut globals.query, &mut globals.arg_numbers, "put", put,3);
    add_command(&mut globals.query, &mut globals.arg_numbers, "set", set,2);
    add_command(&mut globals.query, &mut globals.arg_numbers, "post", post,0);
    add_command(&mut globals.query, &mut globals.arg_numbers, "out", out,1);
    add_command(&mut globals.query, &mut globals.arg_numbers, "goto", goto,1);
    add_command(&mut globals.query, &mut globals.arg_numbers, "if", if_keyword,4);
    add_command(&mut globals.query, &mut globals.arg_numbers, "ifkey", if_key,2);
    add_command(&mut globals.query, &mut globals.arg_numbers, "op", op,3);
    add_command(&mut globals.query, &mut globals.arg_numbers, "init", init,4);
    add_command(&mut globals.query, &mut globals.arg_numbers, "display", display,0);
    add_command(&mut globals.query, &mut globals.arg_numbers, "set_clear", set_clear,1);
    add_command(&mut globals.query, &mut globals.arg_numbers, "clear", clear,0);
    add_command(&mut globals.query, &mut globals.arg_numbers, "handle_input", handle_events,0);
    add_command(&mut globals.query, &mut globals.arg_numbers, "area", area,5);
    add_command(&mut globals.query, &mut globals.arg_numbers, "get", get,3);
    add_command(&mut globals.query, &mut globals.arg_numbers, "resize", resize,2);
    add_command(&mut globals.query, &mut globals.arg_numbers, "exit", exit_command,1);
    add_command(&mut globals.query, &mut globals.arg_numbers, "release", release,1);
    add_command(&mut globals.query, &mut globals.arg_numbers, "reset", reset,0);
    add_command(&mut globals.query, &mut globals.arg_numbers, "sleep", sleep,1);
    add_command(&mut globals.query, &mut globals.arg_numbers, "rng", rng,3);
    add_command(&mut globals.query, &mut globals.arg_numbers, "badduck", badduck,0);
    add_command(&mut globals.query, &mut globals.arg_numbers, "dorbell", dorbell,0);
    add_command(&mut globals.query, &mut globals.arg_numbers, "astrosam", astrosam,0);
    add_command(&mut globals.query, &mut globals.arg_numbers, "zayther", zayther,0);
    add_command(&mut globals.query, &mut globals.arg_numbers, "ovid", ovid,0);
    add_command(&mut globals.query, &mut globals.arg_numbers, "blid", blid,0);

    // Loop variables
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut counter = 0;

    // Foreach line in file...
    for line in reader.lines() {
        let command = line.unwrap();

        // If line is not empty...
        if !command.trim().is_empty() {
            
            // If it's a label...
            if command.trim().starts_with('#') && command.trim().ends_with(':') {
                let label_name : &str = &command.trim()[1..command.trim().len()-1];
                
                // If no other label of that name exists...
                if let None = globals.labels.get(label_name) {
                    // Add to list of labels
                    globals.labels.insert(label_name.to_string(), counter);
                }
                else {
                    panic!("ERR: Label [{}] already exists", label_name);
                }

                globals.commands.push(vec![]);
            }

            // If it's a command...
            else{
                
                string_to_command(&mut globals.arg_numbers,&mut globals.commands, &command);
            }
        }
        // If line is empty
        else {
            globals.commands.push(vec![]);
            
        }

        counter+=1;
    }
    

    
    // While commands pointer is not maxed out or larger than commands array length...
    while globals.cursor != usize::MAX && globals.cursor < globals.commands.len()
    {
        // Grab command
        let command = &globals.commands[globals.cursor].clone();
        match command.len()
        {
            0=>{},
            1=>run(&mut globals, &command[0], None),
            _=>run(&mut globals, &command[0], Some(command[1..].to_vec()))
        }

        globals.cursor+=1;
    }
}


 

