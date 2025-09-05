use sdl3::EventPump;
use sdl3::video::Window;
use sdl3::render::Canvas;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;

use crate::game_state::GameState;
use crate::renderer::Renderer;
#[cfg(feature = "debug-video-to-png")]
use crate::frame_capture::FrameCapture;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;

pub struct App {
    canvas: Canvas<Window>,
    game_state: GameState,
    event_pump: EventPump,
    renderer: Renderer,
    #[cfg(feature = "debug-video-to-png")]
    frame_capture: Option<FrameCapture>,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let sdl_context = sdl3::init()?;
        let video_subsystem = sdl_context.video()?;

        sdl3::hint::set("SDL_RENDER_VSYNC", "1");

        let window = video_subsystem.window(
            "gfx-test",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
        .fullscreen()
        .build()?;

        let canvas = window.into_canvas();

        let game_state = GameState::new();
        let event_pump = sdl_context.event_pump()?;
        let renderer = Renderer::new();

        Ok(Self {
            canvas,
            game_state,
            event_pump,
            renderer,
            #[cfg(feature = "debug-video-to-png")]
            frame_capture: Some(FrameCapture::new()),
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut accumulator = 0.0;
        let timestep = 1.0 / 60.0;

        let mut last_time = sdl3::timer::performance_counter();
        let perf_freq = sdl3::timer::performance_frequency() as f64;

        'running: loop {
            let current_time = sdl3::timer::performance_counter();
            let delta_time = (current_time - last_time) as f64 / perf_freq;
            last_time = current_time;

            accumulator += delta_time;

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }

            let (width, height) = self.canvas.output_size().map_err(|e| e.to_string())?;

            while accumulator >= timestep {
                self.game_state.update(width, height);
                accumulator -= timestep;
            }

            self.renderer.draw(&mut self.canvas, &self.game_state)?;

            #[cfg(feature = "debug-video-to-png")]
            {
                if let Some(fc) = &mut self.frame_capture {
                    fc.capture_frame(self.game_state.frame_counter, width, height, &mut self.canvas)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(feature = "debug-video-to-png")]
impl Drop for App {
    fn drop(&mut self) {
        if let Some(fc) = &self.frame_capture {
            fc.save_frames();
        }
    }
}