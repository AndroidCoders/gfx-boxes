use sdl3::rect::Rect;
use crate::config::ObjectConfig;
use crate::physics::{update_objects, resolve_object_collisions};

pub struct GameObject {
    pub rect: Rect,
    pub velocity: (i32, i32),
}

pub struct GameState {
    pub objects: Vec<GameObject>,
    pub frame_counter: u32,
}

impl GameState {
    pub fn new(objects_config: &[ObjectConfig]) -> Self {
        let objects = objects_config.iter().map(|o| GameObject {
            rect: Rect::new(o.x, o.y, o.width, o.height),
            velocity: (o.vx, o.vy),
        }).collect();

        Self {
            objects,
            frame_counter: 0,
        }
    }

    /// Updates the state of all game objects.
    ///
    /// This includes advancing their positions based on velocity and handling
    /// collisions with the boundaries of the given `width` and `height`.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the game area.
    /// * `height` - The height of the game area.
pub fn update(&mut self, width: u32, height: u32) {
        self.frame_counter += 1;
        update_objects(&mut self.objects, width, height);
        resolve_object_collisions(&mut self.objects);
    }
}
