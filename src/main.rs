// Helper modules
mod graphics;
mod memory;
mod macros;

// Command modules
mod commands;


// Includes
use device_query::DeviceState;
use memory::*;
use graphics::*;
use core::panic;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

// Command includes
use commands::{
    printing::*,
    variables::*,
    con_and_nav::*,
    graphics_and_input::*,
    miscellaneous::*,
    easter_eggs::*,
};

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
    
    add_commands(&mut globals);


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


fn add_commands(globals : &mut Globals)
{
    commands![
        (globals.query, globals.arg_numbers),
        {
            alive =>( alive,0),
            print =>( print,1),
            cursor =>( cursor,0),
            set =>( set,2),
            put =>( put,3),
            out =>( out,1),
            post =>( post,0),
            if =>( if_keyword,4),
            goto =>( goto,1),
            op =>( op,3),
            ifkey =>( if_key,2),
            display =>( display,0),
            init =>( init,4),
            clear =>( clear,0),
            set_clear =>( set_clear,1),
            area =>( area,5),
            handle_input =>( handle_events,0),
            resize =>( resize,2),
            get =>( get,3),
            release =>( release,1),
            exit =>( exit_command,1),
            sleep =>( sleep,1),
            reset =>( reset,0),
            badduck =>( badduck,0),
            rng =>( rng,3),
            astrosam =>( astrosam,0),
            dorbell =>( dorbell,0),
            ovid =>( ovid,0),
            zayther =>( zayther,0),
            blid =>( blid,0),
        }
    ];
}

 

