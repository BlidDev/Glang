/*=====================*/
/*     Conditions &    */  
/*     Navigations     */
/*=====================*/

use std::mem::discriminant;

use device_query::Keycode;

use crate::memory::{Globals, Args, get_var, Types, run};

// Jumps to a label
pub fn goto(globals: &mut Globals, args: Args) {
    let label_name = get_var(&globals.stack, &args.as_ref().unwrap()[0]);
    if let Types::STR(label_name) = label_name {
    globals.cursor = *globals
        .labels
        .get(&label_name)
        .expect(&format!("ERR: Label [{}] does not exist", label_name));
    }
}

// Used in `if` statements
// Checks the two given types and determines
// if the statement is true or false
pub fn statement(first: &Types, second: &Types, op: &str) -> bool {
    if let (Types::I32(a), Types::I32(b)) = (first, second) {
        match op {
            ">" => return a > b,
            "<" => return a < b,
            "<=" => return a <= b,
            ">=" => return a >= b,
            "==" => return a == b,
            "!=" => return a != b,
            _ => panic!("ERR: Unknown operation [{}]", op),
        }
    } else if let (Types::F32(a), Types::F32(b)) = (first, second) {
        match op {
            ">" => return a > b,
            "<" => return a < b,
            "<=" => return a <= b,
            ">=" => return a >= b,
            "==" => return a == b,
            "!=" => return a != b,
            _ => panic!("ERR: Unknown operation [{}]", op),
        }
    } else if let (Types::BOOL(a), Types::BOOL(b)) = (first, second) {
        match op {
            ">" => return a > b,
            "<" => return a < b,
            "<=" => return a <= b,
            ">=" => return a >= b,
            "==" => return a == b,
            "!=" => return a != b,
            _ => panic!("ERR: Unknown operation [{}]", op),
        }
    } else if let (Types::STR(a), Types::STR(b)) = (first, second) {
        match op {
            ">" => return a > b,
            "<" => return a < b,
            "<=" => return a <= b,
            ">=" => return a >= b,
            "==" => return a == b,
            "!=" => return a != b,
            _ => panic!("ERR: Unknown operation [{}]", op),
        }
    } else {
        panic!(
            "ERR: Can't compare type [{:?}] and [{:?}]",
            discriminant(first),
            discriminant(second)
        )
    }
}

// Checks if the given statement is true
pub fn if_keyword(globals: &mut Globals, args: Args) {
        
    let first = get_var(&mut globals.stack, &args.as_ref().unwrap()[0]);
    let op = &args.as_ref().unwrap()[1];
    let second = get_var(&mut globals.stack, &args.as_ref().unwrap()[2]);
    let mut range = 0;
    if let Types::I32(r) = get_var(&mut globals.stack, &args.as_ref().unwrap()[3]) {
        range = (r).max(1) as usize;
    }
    if statement(&first, &second, op){
        let border = globals.cursor + range;
        globals.cursor+=1;
        while globals.cursor < border
        {
            let command = &globals.commands[globals.cursor].clone();
            match command.len() {
                0 => {}
                1 => run(globals, &command[0], None),
                _ => run(globals, &command[0], Some(command[1..].to_vec())),
            }
            globals.cursor +=1;
        }  
          
    
        
    } else {
        globals.cursor += range ;
    }
}

// Checks if a specified key is held down
pub fn if_key(globals: &mut Globals, args: Args) {
    let key = get_var(&mut globals.stack, &args.as_ref().unwrap()[0]);
    let range = get_var(&mut globals.stack, &args.as_ref().unwrap()[1]);
    
    if let (Types::STR(key), Types::I32(range)) =  (key,range){
        let keycode  : Keycode = (&key).parse().expect("ERR: Invalid keycode {}");
    
        if globals.keys.contains(&keycode) == true{
            let border = globals.cursor + range as usize;
            globals.cursor+=1;
            while globals.cursor < border
            {
                let command = &globals.commands[globals.cursor].clone();
                match command.len() {
                    0 => {}
                    1 => run(globals, &command[0], None),
                    _ => run(globals, &command[0], Some(command[1..].to_vec())),
                }
                globals.cursor +=1;
            }  
            
        
            
        } else {
            globals.cursor += range as usize;
        }
    }
}

