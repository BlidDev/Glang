use std::mem::discriminant;

use crate::memory::{get_var_e, Args, Globals,is_variable, parse_variable, Types, get_var, run};
use unescape::unescape;

pub fn alive(_: &mut Globals, _: Args) {
    println!("alive!");
}

pub fn print(_: &mut Globals, args: Args) {
    let s = args.unwrap()[0].clone();
    println!("{}", unescape(&s).expect("ERR: Print value invalid"));
}        


pub fn post(globals: &mut Globals, _: Args)
{
     println!("{:#?}", globals.stack);
}


pub fn out(globals: &mut Globals, _args: Args)
{
     let value =  _args.unwrap()[0].clone();
     if is_variable(&value)
     {
          //println!("{}: {:?}",value[1..].to_string(), globals.stack.get(&value[1..]).expect(&format!("ERR: No variable named [{}]",value[1..].to_string())));
          match globals.stack.get(&value[1..]).expect(&format!("ERR: No variable named [{}]",value[1..].to_string()))
          {
               Types::I32(v)=>print!(" {} ",v),
               Types::F32(v)=>print!(" {} ",v),
               Types::BOOL(v)=>print!(" {} ",v),
               Types::STR(v)=>print!(" {} ",v),
          }
     }
     else
     {
          println!("{:?}",parse_variable(&value));
     }
}

pub fn put(globals: &mut Globals, _args: Args) {
     let args = _args.unwrap().clone();
     
     if let 
     (Types::I32(x),Types::I32(y)) = 
     (get_var_e(&globals.stack, &args[0], &Types::I32(0)), get_var_e(&globals.stack, &args[1], &Types::I32(0))) 
     {
          println!("INFO: Put a pixel at ({},{})",x,y);
     }

}

// set var, 4.5
// set var2, $var
pub fn set(globals: &mut Globals, _args: Args) {
     let args = _args.unwrap().clone();
     if args.len() != 2
     {
          panic!("ERR: Set requires 2 arguments but [{}] were provided",args.len());
     }

     let name = args[0].clone();
     let str_value = args[1].clone();
     
     if is_variable(&str_value)
     {
          let value_name = &str_value[1..];
          let value = globals.stack.get(value_name).expect(&format!("ERR: No variable named [{}]", value_name)).clone();
          *globals.stack.entry(name).or_insert(value.clone()) = value.clone();
     }
     else {
          let value = parse_variable(&str_value);
          *globals.stack.entry(name).or_insert(value.clone()) = value.clone();
     }

}

pub fn goto(globals : &mut Globals, args: Args)
{
     let label_name = &args.unwrap()[0];
     globals.cursor = *globals.labels.get(label_name).expect(&format!("ERR: Label [{}] does not exist", label_name));
}



pub fn statement(first : &Types, second : &Types, op : &str)->bool
{
     if let (Types::I32(a),Types::I32(b)) = (first,second)
     {
          match op {
               ">" => return a > b,
               "<" => return a < b,
               "<=" => return a <= b,
               ">=" => return a >= b,
               "==" => return a == b,
               "!=" => return a != b,
              _=>panic!("ERR: Unknown operation [{}]",op)
          }
     }
     else if let (Types::F32(a),Types::F32(b)) = (first,second)
     {
          match op {
               ">" => return a > b,
               "<" => return a < b,
               "<=" => return a <= b,
               ">=" => return a >= b,
               "==" => return a == b,
               "!=" => return a != b,
              _=>panic!("ERR: Unknown operation [{}]",op)
          }

     }
     else {
         panic!("ERR: Can't compare type [{:?}] and [{:?}]",discriminant(&first),discriminant(&second))
     }

     
}

pub fn if_keyword(globals : &mut Globals, _args: Args)
{
     //println!("{:?}",_args);
     let args = _args.unwrap().clone();
     let first  = get_var(&mut globals.stack, &args[0]);
     let op = args[1].clone();
     let second = get_var(&mut globals.stack, &args[2]);
     let mut range  = 0;
     if let Types::I32(r) = get_var(&mut globals.stack, &args[3]){
          range = r as usize;          
     }

     if statement(&first, &second, &op)
     {
          globals.cursor+=1;
          for _ in globals.cursor..globals.cursor+range-1
          {
              // println!("{}",globals.cursor);
               let command = &globals.commands[globals.cursor].clone();
               match command.len()
               {
                   0=>{},
                   1=>run(globals, &command[0], None),
                   _=>run(globals, &command[0], Some(command[1..].to_vec()))
               }
       
               globals.cursor+=1;
          }
     }
     else {
         globals.cursor += range;
     }
}


pub fn op(globals : &mut Globals, _args : Args)
{
    let args = _args.unwrap().clone();
    let second = get_var(&globals.stack, &args[2]);
    let mut first = globals.stack.get_mut(&args[0][1..]).expect(&format!("ERR: No variable named [{}]", args[0]));
    let op = args[1].as_str();


    if let (Types::I32(a),Types::I32(b)) = (&first,&second)
    {
         //println!("({} {})",a,b);
         match op {
              
              "+"=> *first = Types::I32(a+b),
              "-"=> *first = Types::I32(a-b),
              "*"=> *first = Types::I32(a*b),
              "/"=> *first = Types::I32(a/b),
              
             _=>panic!("ERR: Unknown operation [{}]",op)
         }
    }
    else if let (Types::F32(a),Types::F32(b)) = (&first,&second)
    {
     match op {
              
          "+"=> *first = Types::F32(a+b),
          "-"=> *first = Types::F32(a-b),
          "*"=> *first = Types::F32(a*b),
          "/"=> *first = Types::F32(a/b),
          
         _=>panic!("ERR: Unknown operation [{}]",op)
     }

    }
    else {
        panic!("ERR: Can't oparate type [{:?}] and [{:?}]",discriminant(&first),discriminant(&second))
    }
    
}
