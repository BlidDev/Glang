mod graphics;
mod commands;
mod memory;
use commands::*;
use memory::*;
use graphics::*;
use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};






fn main() {
    let mut globals = Globals
    {
        query : Query::new(),
        commands : vec![],
        stack  : Stack::new(),
        labels  : HashMap::new(),
        cursor : 0,
        graphics : Graphics::default()
    };
    //std::env::set_var("RUST_BACKTRACE", "1");
    add_command(&mut globals.query, "alive", alive);
    add_command(&mut globals.query, "cursor", cursor);
    add_command(&mut globals.query, "print", print);
    add_command(&mut globals.query, "put", put);
    add_command(&mut globals.query, "set", set);
    add_command(&mut globals.query, "post", post);
    add_command(&mut globals.query, "out", out);
    add_command(&mut globals.query, "goto", goto);
    add_command(&mut globals.query, "if", if_keyword);
    add_command(&mut globals.query, "op", op);
    add_command(&mut globals.query, "init", init);
    add_command(&mut globals.query, "display", display);
    add_command(&mut globals.query, "set_clear", set_clear);
    add_command(&mut globals.query, "clear", clear);
    add_command(&mut globals.query, "handle_input", handle_events);
    add_command(&mut globals.query, "area", area);
    add_command(&mut globals.query, "get", get);
    add_command(&mut globals.query, "resize", resize);
    add_command(&mut globals.query, "exit", exit_command);


    //add_command(&mut globals.query, "put", put);
    let file = File::open("res/mockup.glg").unwrap();
    let reader = BufReader::new(file);

    //graphics.init(&mut "Yo".to_string(), (848,480), (212,120));

    //let mut keyboard = buttons::winit_support::keyboard();
    
  
    let mut counter = 0;
    for line in reader.lines() 
    {
        let command = line.unwrap();
        if !command.trim().is_empty() 
        {
            if command.trim().starts_with('#') && command.trim().ends_with(':')
            {
                let label_name : &str = &command.trim()[1..command.trim().len()-1];
                if let None = globals.labels.get(label_name)
                {
                    globals.labels.insert(label_name.to_string(), counter);
                }
                else {
                    panic!("ERR: Label [{}] already exists",label_name);
                }
                globals.commands.push(vec![]);
            }
            else{
                
                string_to_command(&mut globals.commands, &command);
            }
        }
        else {
            globals.commands.push(vec![]);
            
        }
        counter+=1;
    }
    

    
    
    while globals.cursor != usize::MAX && globals.cursor < globals.commands.len()
    {
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


 

