use std::{mem::discriminant, process::exit};

use crate::memory::{get_var, is_variable, parse_variable, run, Args, Globals, Types};
use beryllium::event::Event;
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
    print!("{}", unescape(&s).expect("ERR: Print value invalid"));
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
    let label_name = get_var(&globals.stack, &args.as_ref().unwrap()[0]);
    if let Types::STR(label_name) = label_name {
    globals.cursor = *globals
        .labels
        .get(&label_name)
        .expect(&format!("ERR: Label [{}] does not exist", label_name));
    }
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
            discriminant(first),
            discriminant(second)
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
        range = (r).max(1) as usize;
    }
    if statement(&first, &second, &op) == true{
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
            discriminant(first),
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

     if let (Types::STR(mut n), Types::I32(ww), Types::I32(wh), Types::I32(cw), Types::I32(ch)) 
          = (name, window_w, window_h, canvas_w, canvas_h) 
     {
          globals.graphics.init(&mut n, (ww,wh), (cw as u32, ch as u32))
          .expect("ERR: Could not initialize graphics");
     }
}


pub fn int_to_rgb(color : i32)->(u8,u8,u8)
{
     return (((color>>16)& 0xFF) as u8,((color>>8)& 0xFF) as u8,((color)& 0xFF) as u8)
}

pub fn rgb_to_int(color : (u8,u8,u8))-> i32
{
    let mut i = color.0 as i32;
    i = (i<<8)  + color.1 as i32;
    i = (i<<8)  + color.2 as i32;

    return i;
}

fn set_pixel(buffer : &mut [u8], index : usize, color : &[u8;4])
{
     //println!("{index:?}");
     buffer.chunks_exact_mut(4).nth(index).expect("ERR: Pixel index is out of range").copy_from_slice(color);
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

    if let (Types::I32(mut x), Types::I32(mut y), Types::I32(color)) = (x, y, color) {
        let (w, h) = globals.graphics.canvas_size;
        let (r,g,b) = int_to_rgb(color);
        x = x.clamp(0, (w-1) as i32);
        y = y.clamp(0, (h-1) as i32);
        let index = ( x + w as i32 * y) as usize;
       set_pixel(globals.graphics.pixels.as_mut().unwrap().get_frame(), index, &[r,g,b,255]);
    }
}

pub fn area(globals: &mut Globals, args: Args) 
{
    is_inited(globals.graphics.is_inited, "area");
    let x = get_var(&globals.stack, &args.as_ref().unwrap()[0]);
    let y = get_var(&globals.stack, &args.as_ref().unwrap()[1]);
    let w = get_var(&globals.stack, &args.as_ref().unwrap()[2]);
    let h = get_var(&globals.stack, &args.as_ref().unwrap()[3]);
    let color = get_var(&globals.stack, &args.as_ref().unwrap()[4]);
    if let (Types::I32(x),Types::I32(y),Types::I32(w),Types::I32(h),Types::I32(color)) = 
            (x,y,w,h,color)
    {
        let (cw,ch) = globals.graphics.canvas_size;
        
        for pos_x in x.max(0)..(x.max(0)+w.max(1)).min(cw as i32)
        {
            for pos_y in y.max(0)..(y.max(0)+h.max(1)).min(ch as i32)
            {
                let index = ((pos_x as i32) + cw as i32 * (pos_y as i32)) as usize;
                let (r,g,b) = int_to_rgb(color);
                set_pixel(globals.graphics.pixels.as_mut().unwrap().get_frame(), index, &[r,g,b,255])
            }
        }
    }

}


pub fn get(globals: &mut Globals, args: Args) 
{
     is_inited(globals.graphics.is_inited, "get");
     let x = get_var(&globals.stack, &args.as_ref().unwrap()[0]);
     let y = get_var(&globals.stack, &args.as_ref().unwrap()[1]);
     let name = &args.as_ref().unwrap()[2];

    if let (Types::I32(mut x),Types::I32(mut y)) = (x,y)
    {
        let (w, h) = globals.graphics.canvas_size;
        x = x.clamp(0, (w -1) as i32);
        y = y.clamp(0, (h -1) as i32);
        let index = (x + w as i32 * y) as usize;
        let pixel = globals.graphics.pixels.as_mut().unwrap().get_frame().chunks_exact(4).nth(index).expect("ERR: Could not extract pixel");
        let rgb = rgb_to_int((pixel[0],pixel[1],pixel[2]));

        *globals.stack.entry(name.to_string()).or_insert(Types::I32(rgb)) = Types::I32(rgb);
        
    }
}
pub fn display(globals: &mut Globals, _: Args)
{
     is_inited(globals.graphics.is_inited, "display");

     globals.graphics.pixels.as_ref().unwrap().render().expect("ERR: Could not render pixels");
     
}

pub fn set_clear(globals: &mut Globals, args: Args)
{
    is_inited(globals.graphics.is_inited, "set_clear");
    if let Types::I32(color) = get_var(&globals.stack,&args.as_ref().unwrap()[0]){
        let (r,g,b) = int_to_rgb(color);
        for pixel in globals.graphics.cleanup_buffer.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[r,g,b,255u8]);
        }
    }
}

pub fn clear(globals: &mut Globals, _: Args)
{
     is_inited(globals.graphics.is_inited, "clear");
     globals.graphics.pixels.as_mut().unwrap().get_frame().copy_from_slice(globals.graphics.cleanup_buffer.as_slice());
}


pub fn handle_events(globals: &mut Globals, _: Args)
{
    is_inited(globals.graphics.is_inited, "handle_input");
    while let Some(event) = globals.graphics.sdl_context.as_ref().unwrap().poll_event() {
          match event {
               Event::Quit { .. }=> exit(0),

                Event::WindowResized { width, height, .. } => globals.graphics.pixels.as_mut().unwrap().resize_surface(width, height),
               _=>(),
          }
     }
}


pub fn resize(globals: &mut Globals, args: Args)
{
    is_inited(globals.graphics.is_inited, "resize");
    let w = get_var(&globals.stack, &args.as_ref().unwrap()[0]);
    let h = get_var(&globals.stack, &args.as_ref().unwrap()[1]);

    if let (Types::I32(mut w),Types::I32(mut h)) = (w, h)
    {
        w = w.max(1);
        h = h.max(1);
        let window_size = globals.graphics.window_size;
        let canvas_size = (w,h);
        globals.graphics.canvas_size = (w as u32,h as u32);
        globals.graphics.pixels.as_mut().unwrap().resize_buffer(w as u32, h as u32);
        globals.graphics.pixels.as_mut().unwrap().resize_surface(window_size.0 as u32, window_size.1 as u32);
        
        let r = globals.graphics.cleanup_buffer[0];
        let g = globals.graphics.cleanup_buffer[1];
        let b = globals.graphics.cleanup_buffer[2];
        globals.graphics.cleanup_buffer.resize((canvas_size.0 * canvas_size.1 * 4) as usize, 255u8);

        for i_pixel in globals.graphics.cleanup_buffer.chunks_exact_mut(4)
        {
            i_pixel.copy_from_slice(&[r,g,b,255u8]);
        }
        globals.graphics.pixels.as_mut().unwrap().get_frame().copy_from_slice(globals.graphics.cleanup_buffer.as_slice());
    }
    

}


pub fn exit_command(globals: &mut Globals, args: Args)
{
    if let Types::I32(code) = get_var(&globals.stack, &args.as_ref().unwrap()[0])
    {
        exit(code);
    }
}