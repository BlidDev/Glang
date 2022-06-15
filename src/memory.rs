
use std::{collections::HashMap, mem::discriminant};
use device_query::{DeviceState, Keycode};

use crate::graphics::Graphics;


pub type Args = Option<Vec<String>>;
pub type Query = HashMap<String, fn(&mut Globals, Args)>;
pub type Stack = HashMap<String, Types>;


pub struct Globals {
    pub query: Query,
    pub arg_numbers : HashMap<String,i32>,
    pub commands: Vec<Vec<String>>,
    pub stack: Stack,
    pub labels: HashMap<String, usize>,
    pub cursor: usize,
    pub graphics : Graphics,
    pub keyboard : DeviceState,
    pub keys : Vec<Keycode>,
}

pub fn add_command(query: &mut Query, arg_numbers : &mut HashMap<String,i32>, name: &str, command: fn(&mut Globals, Args), arg_num : i32) {
    
    match query.get(name) {
        Some(_) => println!("command [{}] already exists!", name),
        None => {
            query.insert(name.to_string(), command);
            arg_numbers.insert(name.to_string(), arg_num.max(0));
        }
        
    }
}

pub fn run(globals: &mut Globals, name: &str, args: Args) {
    //println!("{{{}}}",globals.cursor);
    globals
        .query
        .get(name)
        .unwrap()(globals, args);
}

pub fn string_to_command(arg_numbers : &mut HashMap<String,i32>, lines: &mut Vec<Vec<String>>, command: &String) {
    if !command.trim()[0..2].contains("//") { // is a comment
        if command.trim().contains(" ") { // does have parameters
            let parts = command.trim().split_once(" ").unwrap();
            let mut arguments: Vec<String> = parts
                .1                              // a
                .to_string()                    // h   
                .split(",")                     // h
                .map(|s| s.trim().to_string())  // h                   
                .collect();                     // h
            let mut v: Vec<String> = vec![parts.0.to_string()];
            if !arg_numbers.contains_key(&v[0])
            {
                panic!("ERR: Command [{}] does not exist",v[0]);
            }
            else if *arg_numbers.get(&v[0]).unwrap() != arguments.len() as i32
            {
                panic!("ERR: The command [{}] requires [{}] arguments but [{}] were provided",
                        v[0], arg_numbers.get(&v[0]).unwrap(), arguments.len());
            }

            v.append(&mut arguments);

            lines.push(v);
        } else {
            let command = command.trim().to_string();
            if !arg_numbers.contains_key(&command)
            {
                panic!("ERR: Command [{}] does not exist",command);
            }
            else if *arg_numbers.get(&command).unwrap() != 0
            {
                panic!("ERR: The command [{}] requires [{}] arguments but [{}] were provided",
                        command, arg_numbers.get(&command).unwrap(), 0);
            }
            lines.push(vec![command])
        }
    } 
    else {
        lines.push(vec![])
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Types {
    I32(i32),
    F32(f32),
    BOOL(bool),
    STR(String),
}

pub fn is_variable(value: &str) -> bool {
    let chars: Vec<char> = value.chars().collect();
    return chars[0] == '$';
}

pub fn _get_var_e(stack: &Stack, value: &str, kind: &Types) -> Types {
    if is_variable(value)
    //is variable
    {
        let var_name: String = value.chars().collect::<Vec<char>>()[1..].iter().collect();
        let var = stack
            .get(&var_name)
            .expect(&format!("ERR: No variable named [{}]", var_name))
            .clone();

        if discriminant(&var) == discriminant(kind) {
            return var;
        } else {
            panic!(
                "ERR: the vairable [{}], is not the requested type [{:?}]",
                var_name,
                std::mem::discriminant(kind)
            )
        }
    } else
    // a normal value
    {
        let var = parse_variable(value);
        if discriminant(&var) == discriminant(kind) {
            return var;
        } else {
            panic!(
                "ERR: the value [{}], is not the requested type [{:?}]",
                value,
                std::mem::discriminant(kind)
            )
        }
    }
}

pub fn get_var(stack: &Stack, value: &str) -> Types {
    if is_variable(&value) {
        let name = value[1..].to_string();
        return stack
            .get(&name)
            .expect(&format!("ERR: Variable [{}], does not exist", name))
            .clone();
    } else {
        return parse_variable(&value);
    }
}

pub fn parse_variable(value: &str) -> Types {
    if let Ok(i) = value.parse::<i32>() {
        return Types::I32(i);
    } else if let Ok(f) = value.parse::<f32>() {
        return Types::F32(f);
    } else if let Ok(b) = value.parse::<bool>() {
        return Types::BOOL(b);
    }
    return Types::STR(value.to_string());
}
