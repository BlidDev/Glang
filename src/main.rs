mod graphics;
mod commands;
mod memory;
use commands::*;
use memory::*;
use graphics::*;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Texture;
use sdl2::pixels::Color;
use core::panic;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::rc::Rc;
use std::sync::Mutex;




thread_local! 
{
     pub static GRAPHICS : RefCell<Graphics<'static,'static>> = RefCell::new(Graphics::default());
    //pub static TEXTURE : RefCell<Option<Texture<'static>>>  = RefCell::new(None);
}

fn main() {
    let mut globals = Globals
    {
        query : Query::new(),
        commands : vec![],
        stack  : Stack::new(),
        labels  : HashMap::new(),
        cursor : 0,
        //graphics : Graphics::default()
    };
    
    add_command(&mut globals.query, "alive", alive);
    add_command(&mut globals.query, "print", print);
    add_command(&mut globals.query, "put", put);
    add_command(&mut globals.query, "set", set);
    add_command(&mut globals.query, "post", post);
    add_command(&mut globals.query, "out", out);
    add_command(&mut globals.query, "goto", goto);
    add_command(&mut globals.query, "if", if_keyword);
    add_command(&mut globals.query, "op", op);
    let file = File::open("res/mockup.glg").unwrap();
    let reader = BufReader::new(file);

    
   
    GRAPHICS.with(|g|{

        *g.borrow_mut() = Graphics::new("Test",848,480,250,250).unwrap();
        let tc = g.borrow().texture_creator.as_ref().unwrap().create_texture_streaming(PixelFormatEnum::RGB24, 250,250).unwrap();
        g.borrow_mut().texture = Some(&tc);
    });
    //g.create_texture(&mut g.texture_creator.unwrap(),250, 250);
    
  
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



