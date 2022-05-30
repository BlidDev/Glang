use std::{collections::HashMap, mem::discriminant};

pub type Args = Option<Vec<String>>;
pub type Query = HashMap<String, fn(&mut Globals, Args)>;
pub type Stack = HashMap<String, Types>;

pub struct Globals {
    pub query : Query,
    pub commands : Vec<Vec<String>>,
    pub stack: Stack,
    pub labels: HashMap<String, usize>,
    pub cursor: usize,
}

pub fn add_command(query: &mut Query, name: &str, command: fn(&mut Globals, Args)) {
    match query.get(name) {
        Some(_) => println!("command [{}] already exists!", name),
        None => {
            query.insert(name.to_string(), command);
        }
    }
}

pub fn run(globals: &mut Globals, name: &str, args: Args) {
    //println!("{{{}}}",globals.cursor);
    globals.query
        .get(name)
        .expect(&format!("ERR: function [{}] does not exist", &name))(globals, args);
}

pub fn string_to_command(lines: &mut Vec<Vec<String>>, command: &String) {
    if !command.trim()[0..2].contains("//") {
        if command.trim().contains(" ") {
            let parts = command.trim().split_once(" ").unwrap();
            let mut arguments: Vec<String> = parts
                .1
                .to_string()
                .split(",")
                .map(|s| s.trim().to_string())
                .collect();
            let mut v: Vec<String> = vec![parts.0.to_string()];
            v.append(&mut arguments);

            lines.push(v);
        } else {
            lines.push(vec![command.trim().to_string()])
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

pub fn get_var_e(stack: &Stack, value: &str, kind: &Types) -> Types {
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

pub fn get_var(stack: &Stack, value: &str) -> Types 
{
    if is_variable(&value) {
        let name = value[1..].to_string();
        return stack
            .get(&name)
            .expect(&format!("ERR: Variable [{}], does not exist", name)).clone();
    } 
    else {
        return  parse_variable(&value);
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
