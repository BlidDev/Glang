use pixels::{Pixels, SurfaceTexture};
use winit::{window::{Window, WindowBuilder}, dpi::LogicalSize, event_loop::EventLoop};

pub struct Graphics {
    pub window_size: (usize, usize),
    pub canvas_size: (usize, usize),
    pub window: Option<Window>,
    pub pixels: Option<Pixels>,
    pub event_loop : Option<EventLoop<()>>
}

impl Graphics {
    pub fn default() -> Self {
        Graphics {
            window_size: (848, 480),
            canvas_size: (212, 120),
            window: None,
            pixels: None,
            event_loop : None
        }
    }

    pub fn init(&mut self, title: &str, window_size: (usize, usize), canvas_size: (usize, usize)) 
    {
        self.event_loop = Some(EventLoop::new());
        self.window = Some({
            
            let size = LogicalSize::new(window_size.0 as f64, window_size.1 as f64);
            WindowBuilder::new()
                .with_title(title)
                .with_resizable(false)
                .with_inner_size(size)
                .build(&self.event_loop.as_ref().unwrap())
                .unwrap()
        });

        self.pixels = Some({
            let surface_texture = SurfaceTexture::new(canvas_size.0 as u32,canvas_size.1 as u32, self.window.as_ref().unwrap());
            Pixels::new(canvas_size.0 as u32, canvas_size.1 as u32, surface_texture).unwrap()
        });
        
    }
}
