use sdl3::rect::Rect;

pub struct GameObject {
    pub rect: Rect,
    pub velocity: (i32, i32),
}

pub struct GameState {
    pub objects: Vec<GameObject>,
    pub frame_counter: u32,
}

impl GameState {
    pub fn new() -> Self {
        let objects = vec![
            GameObject {
                rect: Rect::new(100, 100, 100, 100),
                velocity: (1, 0),
            },
            GameObject {
                rect: Rect::new(200, 200, 50, 50),
                velocity: (2, 1),
            },
            GameObject {
                rect: Rect::new(300, 300, 75, 75),
                velocity: (-1, -1),
            },
        ];
        Self {
            objects,
            frame_counter: 0,
        }
    }

    pub fn update(&mut self, width: u32, height: u32) {
        self.frame_counter += 1;
        for object in &mut self.objects {
            let next_x = object.rect.x + object.velocity.0;
            if next_x < 0 || (next_x + object.rect.width() as i32) > width as i32 {
                object.velocity.0 = -object.velocity.0;
            }

            let next_y = object.rect.y + object.velocity.1;
            if next_y < 0 || (next_y + object.rect.height() as i32) > height as i32 {
                object.velocity.1 = -object.velocity.1;
            }

            object.rect.x += object.velocity.0;
            object.rect.y += object.velocity.1;
        }
    }
}
