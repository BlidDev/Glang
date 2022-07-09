
/*=====================*/
/*    Miscellaneous    */  
/*=====================*/

use std::{thread,time, process::exit};

use crate::memory::{Globals, Args, get_var, Types};

// Makes the thread pause for a given amount of milliseconds
pub fn sleep(globals: &mut Globals, args: Args)
{
    if let Types::I32(mut millis) = get_var(&globals.stack, &args.as_ref().unwrap()[0]) {
        millis = millis.max(0); 
        let time = time::Duration::from_millis(millis as u64);
        thread::sleep(time)
    }
}

// Quits the program
pub fn exit_command(globals: &mut Globals, args: Args)
{
    if let Types::I32(code) = get_var(&globals.stack, &args.as_ref().unwrap()[0])
    {
        exit(code);
    }
}