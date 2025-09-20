// src/physics.rs

use crate::game_state::GameObject;

/// Updates the state of all game objects, including movement and wall collisions.
///
/// # Arguments
///
/// * `objects` - A mutable slice of `GameObject`s to update.
/// * `width` - The width of the game area.
/// * `height` - The height of the game area.
pub fn update_objects(objects: &mut [GameObject], width: u32, height: u32) {
    for object in objects.iter_mut() {
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

/// Detects and resolves collisions between objects.
///
/// # Arguments
///
/// * `objects` - A mutable slice of `GameObject`s to check for collisions.
pub fn resolve_object_collisions(objects: &mut [GameObject]) {
    // Simple AABB collision detection and resolution
    for i in 0..objects.len() {
        for j in (i + 1)..objects.len() {
            let (obj1, obj2) = {
                let (left, right) = objects.split_at_mut(j);
                (&mut left[i], &mut right[0])
            };

            if obj1.rect.has_intersection(obj2.rect) {
                // Optional: Separate overlapping objects to prevent sticking
                // This is a simplified approach and can be improved for more realistic physics
                let intersection = obj1.rect.intersection(obj2.rect).unwrap();

                let overlap_x = intersection.width() as i32;
                let overlap_y = intersection.height() as i32;

                if overlap_x < overlap_y {
                    // Collision is more horizontal
                    obj1.velocity.0 = -obj1.velocity.0;
                    obj2.velocity.0 = -obj2.velocity.0;

                    if obj1.rect.x < obj2.rect.x {
                        obj1.rect.x -= overlap_x / 2;
                        obj2.rect.x += overlap_x / 2;
                    } else {
                        obj1.rect.x += overlap_x / 2;
                        obj2.rect.x -= overlap_x / 2;
                    }
                } else {
                    // Collision is more vertical
                    obj1.velocity.1 = -obj1.velocity.1;
                    obj2.velocity.1 = -obj2.velocity.1;

                    if obj1.rect.y < obj2.rect.y {
                        obj1.rect.y -= overlap_y / 2;
                        obj2.rect.y += overlap_y / 2;
                    } else {
                        obj1.rect.y += overlap_y / 2;
                        obj2.rect.y -= overlap_y / 2;
                    }
                }
            }
        }
    }
}
