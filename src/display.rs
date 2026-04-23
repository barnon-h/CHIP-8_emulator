use pixels::{ Pixels, SurfaceTexture };
use winit::dpi::LogicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;
use winit::window::WindowAttributes;
use std::sync::Arc;

pub const SCALE : u32 = 10;
pub const WIDTH : u32 = 64 * SCALE;
pub const HEIGHT : u32 = 32 * SCALE;

// Themes
pub const THEMES: [[ [u8; 4]; 2]; 6] = [
    // black and white
    [[ 0x00, 0x00, 0x00, 0xFF ], [ 0xFF, 0xFF, 0xFF, 0xFF ]],
    // Green
    [[ 0x00, 0x00, 0x00, 0xFF ], [ 0x00, 0xFF, 0x00, 0xFF ]],
    // Amber
    [[ 0x00, 0x00, 0x00, 0xFF ], [ 0xFF, 0x6B, 0x00, 0xFF ]],
    // Synthwave
    [[ 0x1A, 0x1A, 0x2E, 0xFF ], [ 0xE9, 0x4F, 0x6F, 0xFF ]],
    // Cyan neon
    [[ 0x0D, 0x0D, 0x0D, 0xFF ], [ 0x00, 0xF5, 0xFF, 0xFF ]],
    // Game Boy
    [[ 0x0F, 0x38, 0x0F, 0xFF ], [ 0x8B, 0xAC, 0x0F, 0xFF ]],
];

pub const TITLE : &str = "CHIP8 Emulator V1.00";

pub struct Display<'a>
{
    pub pixels: Pixels<'a>,
    pub window: Arc<Window>,
    pub theme: usize,
}

impl<'a> Display<'a>
{
    // Constructor
    pub fn new( event_loop: &ActiveEventLoop ) -> Self
    {
        let attr = WindowAttributes::default()
        .with_title( TITLE )
        .with_inner_size( LogicalSize::new( WIDTH, HEIGHT ))
        .with_resizable( false );

        let window = Arc::new(event_loop.create_window(attr ).unwrap());

        let surface_textures = SurfaceTexture::new( WIDTH, HEIGHT, window.clone() );
        let pixels = Pixels::new( 64, 32, surface_textures ).unwrap();


        Self { pixels, window, theme: 0 }
    }

    // draw window
    pub fn draw( &mut self, display : &[ bool; 64 * 32 ])
    {
        let frame  = self.pixels.frame_mut();
        let [ off, on ] = THEMES[ self.theme ];

        for ( i, pixel ) in display.iter().enumerate()
        {
            let color = if *pixel { on } else { off };
            let offset = i * 4;


            frame[ offset..offset + 4].copy_from_slice( &color );
        }

        // render frame
        self.pixels.render().unwrap()
    }

    // cycle function for themes

    //next theme
    pub fn next_theme( &mut self )
    {
        self.theme = ( self.theme + 1 ) % THEMES.len();
        self.update_title();
    }

    // prev theme
    pub fn prev_theme( &mut self )
    {
        self.theme = if self.theme == 0 { THEMES.len() - 1 } else { self.theme - 1 };
        self.update_title()
    }

    //update title
    fn update_title( &self )
    {
        let names = [
            "Classic",
            "Green Phosphor",
            "Amber Monitor",
            "Synthwave",
            "Cyan Neon",
            "Game Boy",
        ];
        self.window.set_title( &format!("{} — {}", TITLE, names[ self.theme ] ));
    }
}