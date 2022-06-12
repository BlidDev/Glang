mod graphics;
mod commands;
mod memory;
use commands::*;
use memory::*;
use graphics::*;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::Window;
use core::panic;
use std::borrow::BorrowMut;
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
    add_command(&mut globals.query, "clear", clear);
    //add_command(&mut globals.query, "put", put);
    let file = File::open("res/mockup.glg").unwrap();
    let reader = BufReader::new(file);

    let mut graphics = Graphics::default();
    
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
    let mut is_ok = false; // used to check if we initilized graphics or just existed the program

    
    
    while globals.cursor != usize::MAX && globals.cursor < globals.commands.len()
    {
        if globals.graphics.is_inited 
        {
            is_ok = true;
            break;
        }
        let command = &globals.commands[globals.cursor].clone();
        match command.len()
        {
            0=>{},
            1=>run(&mut globals, &command[0], None),
            _=>run(&mut globals, &command[0], Some(command[1..].to_vec()))
        }

        globals.cursor+=1;
    }


    if globals.graphics.is_inited && is_ok
    {
        
        globals.graphics.event_loop.unwrap().run(|event,_,control_flow|{
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == globals.graphics.window.as_ref().unwrap().id() => *control_flow = ControlFlow::Exit,
                _ => (),
            }
        });
    }

}


 

