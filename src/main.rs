mod cpu;
mod display;
mod keyboard;
mod audio;

use cpu::CPU;
use display::Display;
use keyboard::map_key;
use audio::Audio;

use winit::application::ApplicationHandler;
use winit::event::{ WindowEvent, KeyEvent, ElementState };
use winit::event_loop::{ ActiveEventLoop, EventLoop };
use winit::keyboard::{ KeyCode, PhysicalKey };
use winit::window::WindowId;
use std::time::{ Duration, Instant };
struct App
{
    cpu: CPU,
    display: Option< Display< 'static >>,
    audio: Audio,
    last_tick : Instant
}

impl App
{
    // Constructor
    pub fn new() -> Self
    {
        let mut cpu = CPU::new();
        cpu.load_rom("roms/Space Invaders.ch8" );

        Self { cpu: ( cpu ), display: ( None ), audio:( Audio::new() ), last_tick : ( Instant::now() )}
    }
}

impl ApplicationHandler for App
{
    fn resumed( &mut self, event_loop: &ActiveEventLoop )
    {
        self.display = Some( Display::new( event_loop ));

        if let Some(display) = &self.display
        {
            display.window.request_redraw();
        }
    }

    fn about_to_wait( &mut self, event_loop: &ActiveEventLoop )
    {
        let frame_rate = Duration::from_micros( 16667 ); // 60fps
        let elapsed = self.last_tick.elapsed();

        // sleep till frame refresh time
        if elapsed < frame_rate
        {
            std::thread::sleep( frame_rate - elapsed );
        }

        self.last_tick = Instant::now();
        if let Some( display ) = &self.display
        {
            display.window.request_redraw();
        }
    }

    fn window_event( &mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent )
    {
        match event
        {
            // Exit event
            WindowEvent::CloseRequested =>
            {
                event_loop.exit();
            }

            // Keyboard events
            WindowEvent::KeyboardInput { event : KeyEvent{ physical_key: PhysicalKey::Code( key ), state, ..}, ..} =>
            {
                let pressed = state == ElementState::Pressed;

                // exit
                if key == KeyCode::Escape{ event_loop.exit(); }

                // CHIP-8 Keyboard
                if let Some( k ) = map_key( key )
                {
                    self.cpu.keyboard[ k ] = pressed;
                }

                // Cycle Themes
                if key == KeyCode::KeyT && pressed
                {
                    if let Some( display ) = &mut self.display
                    {
                        display.next_theme();
                        display.draw( &self.cpu.display );
                    }
                }
            }

            // Draw each Frame
            WindowEvent::RedrawRequested =>
            {
                // 10 cpu cycles per frame
                for _ in 0 .. 10
                {
                    self.cpu.cycle();
                }

                // tick timers at 60hz
                if self.cpu.delay_timer > 0
                {
                    self.cpu.delay_timer -= 1;
                }

                if self.cpu.sound_timer > 0
                {
                    self.cpu.sound_timer -= 1;
                }

                // audio
                if self.cpu.sound_timer > 0
                {
                    self.audio.play();
                }
                else
                {
                    self.audio.stop();
                }

                // redraw if redraw flag
                if self.cpu.draw_flag
                {
                    if let Some( display ) = &mut self.display
                    {
                        display.draw( &self.cpu.display );
                    }

                    self.cpu.draw_flag = false;
                }

                // request next frame
                if let Some( display ) = &mut self.display 
                {
                    display.window.request_redraw();
                }
            }
            _ => {}
        }
    }
}

fn main()
{
    println!(" Starting Chip-8 Emulator");
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new();
    event_loop.run_app( &mut app ).unwrap()
}

