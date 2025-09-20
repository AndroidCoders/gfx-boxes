mod app;
mod game_state;
mod renderer;
mod config;

#[cfg(feature = "debug-video-to-png")]
mod frame_capture;

use app::App;
use config::load_config;

fn main() -> Result<(), String> {
    let config = load_config().map_err(|e| e.to_string())?;
    let mut app = App::new(&config).map_err(|e| e.to_string())?;
    app.run()
}