/*=====================*/
/*      Graphics &     */  
/*    Input Handling   */
/*=====================*/

use std::process::exit;

use beryllium::event::Event;
use device_query::DeviceQuery;

use crate::memory::{Globals, Args, get_var, Types};


// Initializes a new canvas (window)
pub fn init(globals: &mut Globals, args: Args)
{
     
     let window_w = get_var(&mut globals.stack, &args.as_ref().unwrap()[0]);
     let window_h = get_var(&mut globals.stack, &args.as_ref().unwrap()[1]);
     let canvas_w = get_var(&mut globals.stack, &args.as_ref().unwrap()[2]);
     let canvas_h = get_var(&mut globals.stack, &args.as_ref().unwrap()[3]);

     if let (Types::I32(ww), Types::I32(wh), Types::I32(cw), Types::I32(ch)) 
          = (window_w, window_h, canvas_w, canvas_h) 
     {
          globals.graphics.init((ww,wh), (cw as u32, ch as u32))
          .expect("ERR: Could not initialize graphics");
     }
}

// Places a colored pixel at given coordinates
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

// Fills a given area in the canvas with colored pixels
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

// Gets the pixel color at given coordinates
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

// Updates the display
pub fn display(globals: &mut Globals, _: Args)
{
     is_inited(globals.graphics.is_inited, "display");

     globals.graphics.pixels.as_ref().unwrap().render().expect("ERR: Could not render pixels");
     
}

// Sets the background color for `clear`
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

// Clears the canvas
pub fn clear(globals: &mut Globals, _: Args)
{
     is_inited(globals.graphics.is_inited, "clear");
     globals.graphics.pixels.as_mut().unwrap().get_frame().copy_from_slice(globals.graphics.cleanup_buffer.as_slice());
}

// Resizes the window
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

// Handles events, such as keyboard input and graphical changes
pub fn handle_events(globals: &mut Globals, _: Args)
{
    is_inited(globals.graphics.is_inited, "handle_input");
    globals.keys = vec![];
    while let Some(event) = globals.graphics.sdl_context.as_ref().unwrap().poll_event() {
          match event {
               Event::Quit { .. }=> exit(0),

                Event::WindowResized { width, height, .. } => globals.graphics.pixels.as_mut().unwrap().resize_surface(width, height),
                Event::Keyboard {..} => {globals.keys = globals.keyboard.get_keys()},
                _=>(),
          }
    }
}


/*=====================*/
/*   Local Functions   */  
/*=====================*/

// Sets the pixel color at given index
fn set_pixel(buffer : &mut [u8], index : usize, color : &[u8;4])
{
     buffer.chunks_exact_mut(4).nth(index).expect("ERR: Pixel index is out of range").copy_from_slice(color);
}

// Checks if initialized
fn is_inited(is_inited : bool, command : &str)
{
     if !is_inited {
          panic!("ERR: Trying to call graphical command [{}] but graphics is not initialized",command)
     }
}



/*====================*/
/*  Public Functions  */  
/*====================*/

/* 
    RGB-INT Converters
*/
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
