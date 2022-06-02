extern crate sdl2;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};


use sdl2::rect::Rect;
use sdl2::render::Texture;




pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let width  : usize = 848;
    let height : usize  = 480;
    let window = video_subsystem
        .window("Canvas Demo", width as u32, height as u32)
        .position_centered()
        .vulkan()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    
    let mut screen: Vec<u8> = vec![255; width * height * 3];
    let mut event_pump = sdl_context.event_pump()?;
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, width as u32, height as u32)
        .map_err(|e| e.to_string())?;
    update(&mut texture, &mut screen, &width, &height)?;
    let mut counter : f64 =0.0;
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.copy(&texture, None, None)?;
    

    'running: loop {


        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {keycode : Some(Keycode::R),..}=>{
                    screen = vec![255; height * width * 3];
                },
                _ => {}
            }
        }
        update(&mut texture, &mut screen, &width, &height)?;

        canvas.clear();
        //canvas.copy(&texture, None, Rect::new(0, 0, width as u32, height as u32))?;
        canvas.copy_ex(
            &texture,
            None,
            Some(Rect::new(0, 0, width as u32, height as u32)),
            counter.sin() * 5.0,
            None,
            false,
            false,
        )?;

        canvas.present();
        counter += 0.001;
    }

    Ok(())
}

fn update(texture: &mut Texture, buffer: &mut Vec<u8>, width: &usize, height: &usize) -> Result<(), String> {
    let pitch = width * 3;
    let x = rand::thread_rng().gen_range(0..*width);
    let y = rand::thread_rng().gen_range(0..*height);
    let offset = y * pitch + x * 3;
    buffer[offset] = rand::thread_rng().gen_range(0..255);
    buffer[offset + 1] = rand::thread_rng().gen_range(0..255);
    buffer[offset + 2] = rand::thread_rng().gen_range(0..255);
       
    
    texture
        .update(None, unsafe { &buffer.align_to().1 }, width * 3)
        .map_err(|e| e.to_string())?;
    Ok(())
}
