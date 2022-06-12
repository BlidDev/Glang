use std::mem::discriminant;

use crate::memory::{get_var, is_variable, parse_variable, run, Args, Globals, Types};
use unescape::unescape;

pub fn alive(_: &mut Globals, _: Args) {
    println!("alive!");
}
pub fn cursor(globals: &mut Globals, _: Args)
{ 
    
     println!("{}",globals.cursor);
}

pub fn print(_: &mut Globals, args: Args) {
    
    let s = args.unwrap()[0].clone();
    println!("{}", unescape(&s).expect("ERR: Print value invalid"));
}

pub fn post(globals: &mut Globals, _: Args) {
    println!("{:#?}", globals.stack);
}

pub fn out(globals: &mut Globals, _args: Args) {
    let value = _args.unwrap()[0].clone();
    match get_var(&globals.stack,&value) {
        Types::I32(v) => println!("{}",v),
        Types::F32(v) => println!("{}",v),
        Types::BOOL(v) => println!("{}",v),
        Types::STR(v) => println!("{}",v),
    }
}

// set var, 4.5
// set var2, $var
pub fn set(globals: &mut Globals, _args: Args) {
    let args = _args.unwrap().clone();
    if args.len() != 2 {
        panic!(
            "ERR: Set requires 2 arguments but [{}] were provided",
            args.len()
        );
    }

    let name = args[0].clone();
    let str_value = args[1].clone();

    if is_variable(&str_value) {
        let value_name = &str_value[1..];
        let value = globals
            .stack
            .get(value_name)
            .expect(&format!("ERR: No variable named [{}]", value_name))
            .clone();
        *globals.stack.entry(name).or_insert(value.clone()) = value.clone();
    } else {
        let value = parse_variable(&str_value);
        *globals.stack.entry(name).or_insert(value.clone()) = value.clone();
    }
}

pub fn goto(globals: &mut Globals, args: Args) {
    let label_name = &args.unwrap()[0];
    globals.cursor = *globals
        .labels
        .get(label_name)
        .expect(&format!("ERR: Label [{}] does not exist", label_name));
}

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
    } else {
        panic!(
            "ERR: Can't compare type [{:?}] and [{:?}]",
            discriminant(&first),
            discriminant(&second)
        )
    }
}

pub fn if_keyword(globals: &mut Globals, _args: Args) {
    //println!("{:?}",_args);
    let args = _args.unwrap().clone();
    let first = get_var(&mut globals.stack, &args[0]);
    let op = args[1].clone();
    let second = get_var(&mut globals.stack, &args[2]);
    let mut range = 0;
    if let Types::I32(r) = get_var(&mut globals.stack, &args[3]) {
        range = (r -1).max(1) as usize;
    }

    if statement(&first, &second, &op) {
        
        for cursor in globals.cursor+1..globals.cursor + range - 1 {
            //println!("{}",cursor);
            let command = &globals.commands[cursor].clone();
            match command.len() {
                0 => {}
                1 => run(globals, &command[0], None),
                _ => run(globals, &command[0], Some(command[1..].to_vec())),
            }

            globals.cursor += 1;
        }
    } else {
        globals.cursor += range + 1;
    }
}

pub fn op(globals: &mut Globals, _args: Args) {
    let args = _args.unwrap().clone();
    let second = get_var(&globals.stack, &args[2]);
    let first = globals
        .stack
        .get_mut(&args[0][1..])
        .expect(&format!("ERR: No variable named [{}]", args[0]));
    let op = args[1].as_str();

    if let (Types::I32(a), Types::I32(b)) = (&first, &second) {
        //println!("({} {})",a,b);
        match op {
            "+" => *first = Types::I32(a + b),
            "-" => *first = Types::I32(a - b),
            "*" => *first = Types::I32(a * b),
            "/" => *first = Types::I32(a / b),

            _ => panic!("ERR: Unknown operation [{}]", op),
        }
    } else if let (Types::F32(a), Types::F32(b)) = (&first, &second) {
        match op {
            "+" => *first = Types::F32(a + b),
            "-" => *first = Types::F32(a - b),
            "*" => *first = Types::F32(a * b),
            "/" => *first = Types::F32(a / b),

            _ => panic!("ERR: Unknown operation [{}]", op),
        }
    } else {
        panic!(
            "ERR: Can't oparate type [{:?}] and [{:?}]",
            discriminant(&first),
            discriminant(&second)
        )
    }
}

pub fn init(globals: &mut Globals, args: Args)
{
     let name = get_var(&mut globals.stack, &args.as_ref().unwrap()[0]);
     let window_w = get_var(&mut globals.stack, &args.as_ref().unwrap()[1]);
     let window_h = get_var(&mut globals.stack, &args.as_ref().unwrap()[2]);
     let canvas_w = get_var(&mut globals.stack, &args.as_ref().unwrap()[3]);
     let canvas_h = get_var(&mut globals.stack, &args.as_ref().unwrap()[4]);

     if let (Types::STR(n), Types::I32(ww), Types::I32(wh), Types::I32(cw), Types::I32(ch)) 
          = (name, window_w, window_h, canvas_w, canvas_h) 
     {
          globals.graphics.init(&n, (ww as usize, wh as usize), (cw as usize, ch as usize));
     }
}


pub fn int_to_rgb(color : i32)->(u8,u8,u8)
{
     return (((color>>16)& 0xFF) as u8,((color>>8)& 0xFF) as u8,((color)& 0xFF) as u8)
}

fn set_pixel(buffer : &mut [u8], index : usize, color : &[u8;4])
{
     buffer.chunks_exact_mut(4).nth(index).unwrap().copy_from_slice(color);
}

fn is_inited(is_inited : bool, command : &str)
{
     if !is_inited {
          panic!("ERR: Trying to call graphical command [{}] but graphics is not initialized",command)
     }
}

pub fn put(globals: &mut Globals, args: Args) {
    is_inited(globals.graphics.is_inited, "put");

    let x = get_var(&mut globals.stack, &args.as_ref().unwrap()[0].clone());
    let y = get_var(&mut globals.stack, &args.as_ref().unwrap()[1].clone());
    let color = get_var(&mut globals.stack, &args.as_ref().unwrap()[2].clone());

    if let (Types::I32(x), Types::I32(y), Types::I32(color)) = (x, y, color) {
        let (w, h) = globals.graphics.canvas_size;
        let (r,g,b) = int_to_rgb(color);
        let index = (x.clamp(0, w as i32) + w as i32 * y.clamp(0, h as i32)) as usize;
       set_pixel(globals.graphics.pixels.as_mut().unwrap().get_frame(), index, &[r,g,b,255]);
    }
}

pub fn area(globals: &mut Globals, args: Args) 
{
     is_inited(globals.graphics.is_inited, "area");
}
pub fn get(globals: &mut Globals, args: Args) 
{
     is_inited(globals.graphics.is_inited, "get");
}
pub fn display(globals: &mut Globals, args: Args)
{
     is_inited(globals.graphics.is_inited, "display");

     globals.graphics.pixels.as_ref().unwrap().render().expect("ERR: Could not render pixels");
     globals.graphics.window.as_ref().unwrap().request_redraw();
}
pub fn clear(globals: &mut Globals, _: Args)
{
     is_inited(globals.graphics.is_inited, "clear");
     let size : usize = globals.graphics.canvas_size.0 * globals.graphics.canvas_size.1 * 4;
     let vec = vec![0u8;size];
     globals.graphics.pixels.as_mut().unwrap().get_frame().copy_from_slice(vec.as_slice());
}
