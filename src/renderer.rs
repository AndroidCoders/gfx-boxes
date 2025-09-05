use sdl3::render::Canvas;
use sdl3::video::Window;
use sdl3::pixels::Color;
use crate::game_state::GameState;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, game_state: &GameState) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for object in &game_state.objects {
            canvas.fill_rect(object.rect).map_err(|e| e.to_string())?;
        }
        canvas.present();
        Ok(())
    }
}
