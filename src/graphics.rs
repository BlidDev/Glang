use pixels::{Pixels, SurfaceTexture};

use beryllium::{
    init::{InitFlags, Sdl},
    vk_window::VkWindow, window::WindowFlags,
};
use zstring::zstr;

pub struct Graphics {
    pub is_inited: bool,
    pub window_size: (i32, i32),
    pub canvas_size: (u32, u32),
    pub sdl_context: Option<Sdl>,
    pub window: Option<VkWindow>,
    pub pixels: Option<Pixels>,
    pub cleanup_buffer : Vec<u8>,
}

impl Graphics {
    pub fn default() -> Self {
        Graphics {
            is_inited: false,
            window_size: (848, 480),
            canvas_size: (212, 120),
            sdl_context: None,
            window: None,
            pixels: None,
            cleanup_buffer : vec![],
        }
    }

    pub fn init(&mut self, window_size: (i32, i32), canvas_size: (u32, u32)) -> Result<&mut Self, Box<dyn std::error::Error>> {
        self.window_size = window_size;
        self.canvas_size = canvas_size;
        self.cleanup_buffer = vec![0u8; (canvas_size.0 * canvas_size.1 * 4) as usize];
        for pixel in self.cleanup_buffer.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0u8,0u8,0u8,255u8]);
        }
        self.sdl_context = Some(Sdl::init(InitFlags::EVERYTHING)?);
        
        self.window = Some(self.sdl_context.as_ref().unwrap().create_vk_window(
            //ZStr::from_non_null_unchecked(NonNull::new(title.as_mut_ptr()).unwrap()),
            zstr!("Glang Window"),
            None,
            self.window_size,
            WindowFlags::ALLOW_HIGHDPI | WindowFlags::RESIZABLE | WindowFlags::VULKAN,
        )?);
        
        self.pixels = Some(
            {
                let window = self.window.as_ref().unwrap();
                let surface_texture = SurfaceTexture::new(self.canvas_size.0, self.canvas_size.1, &**window);
                Pixels::new(self.canvas_size.0, self.canvas_size.1, surface_texture)?
            }
            
        );
        self.pixels.as_mut().unwrap().resize_surface(self.window_size.0 as u32, self.window_size.1 as u32);
        
        self.is_inited = true;
        Ok(self)
    }
}
