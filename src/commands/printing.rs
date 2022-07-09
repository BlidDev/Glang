/*=====================*/
/*  Printing Commands* */  
/*=====================*/
extern crate unescape;
use crate::memory::{Globals, Args, get_var, Types};
use unescape::unescape;

// Prints the line "alive!"
pub fn alive(_: &mut Globals, _: Args) {
    println!("alive!");
}
// Prints out the command ptr variable
pub fn cursor(globals: &mut Globals, _: Args)
{ 
     
     println!("{}",globals.cursor);
}

// Prints out plain text
pub fn print(_: &mut Globals, args: Args) {
    
    let s = args.unwrap()[0].clone();
    print!("{}", unescape(&s).expect("ERR: Print value invalid"));
}

// Prints out the whole stack
pub fn post(globals: &mut Globals, _: Args) {
    println!("{:#?}", globals.stack);
}
// Prints out a `set` variable
pub fn out(globals: &mut Globals, _args: Args) {
    let value = _args.unwrap()[0].clone();
    match get_var(&globals.stack,&value) {
        Types::I32(v) => println!("{}",v),
        Types::F32(v) => println!("{}",v),
        Types::BOOL(v) => println!("{}",v),
        Types::STR(v) => println!("{}",v),
    }
}