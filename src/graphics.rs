use std::sync::Arc;

use sdl2::{
    pixels::{PixelFormat, PixelFormatEnum},
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
    EventPump, Sdl, VideoSubsystem,
};

pub struct Graphics<'a,'b> {
    pub is_inited: bool,
    pub w: u32,
    pub h: u32,
    pub screen: Vec<u8>,
    pub sdl_context: Option<Sdl>,
    pub video_subsystem: Option<VideoSubsystem>,
    pub texture_creator: Option<TextureCreator<WindowContext>>,
    pub canvas: Option<Canvas<Window>>,

    pub texture: Option<&'b Texture<'a>>,
}

impl<'a,'b> Graphics<'a,'b> {
    pub fn default() -> Self {
        Graphics {
            is_inited: false,
            w: 848,
            h: 480,
            screen: vec![0; 848 * 480 * 3],
            sdl_context: None,
            video_subsystem: None,
            texture_creator: None,
            canvas: None,
            texture: None,
        }
    }

    pub fn new(
        title: &str,
        width: usize,
        height: usize,
        canvas_w: usize,
        canvas_h: usize,
    ) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window(title, width as u32, height as u32)
            .vulkan()
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;

        let texture_creator = canvas.texture_creator();

        let mut g = Graphics {
            is_inited: false,
            w: canvas_w as u32,
            h: canvas_h as u32,
            screen: vec![0; canvas_w * canvas_h * 3],
            sdl_context: Some(sdl_context),
            video_subsystem: Some(video_subsystem),
            texture_creator: Some(texture_creator),
            canvas: Some(canvas),
            texture: None, //texture: Some(texture),
        };

        // let texture  = g.texture_creator
                // .as_ref()
                // .unwrap()
                // .create_texture_streaming(PixelFormatEnum::RGB24, width as u32, height as u32)
                // .unwrap();
        
        //g.create_texture(canvas_w, canvas_h);

        Ok(g)
    }
    pub fn test(&'static mut self)
    {
        println!("{} {}",self.w,self.h)
    }
    pub fn create_texture(&mut self,tc : &'b mut TextureCreator<WindowContext>,width: usize, height: usize) {
       //self.texture = Some(&tc.create_texture_streaming(PixelFormatEnum::RGB24, width as u32, height as u32).unwrap());
    }
}
