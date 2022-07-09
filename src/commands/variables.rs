/*=====================*/
/*  Variable commands  */
/*=====================*/

use std::mem::discriminant;

use rand::{thread_rng, Rng};

use crate::memory::{Globals, Args, is_variable, parse_variable, Stack, get_var, Types};

// Set a new variable
pub fn set(globals: &mut Globals, args: Args) {
    let name = args.as_ref().unwrap()[0].clone();
    let str_value = args.as_ref().unwrap()[1].as_str();

    if is_variable(str_value) {
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

// Free the variable from the stack
pub fn release(globals: &mut Globals, args: Args)
{
    let str_value = &args.as_ref().unwrap()[0];
    if is_variable(&str_value)
    {
        let value_name = &str_value[1..];
        globals.stack.remove(value_name)
                      .expect(&format!("ERR: No variable name [{}]", value_name));
    }
    else {
        panic!("ERR: Can not release, [{}] is not a variable",str_value);
    }
}

// Resets the entire stack
pub fn reset(globals: &mut Globals, _: Args)
{
    globals.stack = Stack::new();
}

// Generates a random number of a given range
pub fn rng(globals: &mut Globals, args: Args)
{
    let name = get_var(&globals.stack, &args.as_ref().unwrap()[0]);
    let start = get_var(&globals.stack, &args.as_ref().unwrap()[1]);
    let end = get_var(&globals.stack, &args.as_ref().unwrap()[2]);
    if let (Types::STR(name),Types::I32(mut start),Types::I32(mut end)) = (&name, &start, &end)
    {
        start = start.max(0);
        end = end.max(0);
        if start < end // valid range
        {
            let rng = thread_rng().gen_range(start..end);
            *globals.stack.entry(name.to_string()).or_insert(Types::I32(rng)) = Types::I32(rng);
        }
        else
        {
            panic!("ERR: Invaild rng range [{}..{}]",start,end);
        }
    }
    else if let (Types::STR(name),Types::F32(mut start),Types::F32(mut end)) = (name, start, end)
    {
        start = start.max(0.0);
        end = end.max(0.0);
        if start < end // valid range
        {
            let rng = thread_rng().gen_range(start..end);
            *globals.stack.entry(name).or_insert(Types::F32(rng)) = Types::F32(rng);
        }
        else
        {
            panic!("ERR: Invaild rng range [{}..{}]",start,end);
        }
    }
}


// Performs arithmetic
pub fn op(globals: &mut Globals, _args: Args) {
    let args = _args.unwrap().clone();
    let second = get_var(&globals.stack, &args[2]);
    let first = globals
        .stack
        .get_mut(&args[0][1..])
        .expect(&format!("ERR: No variable named [{}]", args[0]));
    let op = args[1].as_str();

    if let (Types::I32(a), Types::I32(b)) = (&first, &second) {
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
