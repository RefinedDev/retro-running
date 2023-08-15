use godot::{
    engine::{CharacterBody3D, CharacterBody3DVirtual, Control},
    prelude::*,
};

use crate::main_scene::PlatformDirection;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player {
    pub speed: f64,
    jump_power: f64,
    gravity: f64,
    
    #[export]
    dead: bool,

    platform_standing: PlatformDirection,

    #[base]
    base: Base<CharacterBody3D>,
}

#[godot_api]
impl Player {
    // fn get_collided_objects(&mut self) -> Vec<Gd<Object>> {
    //     let mut objects: Vec<Gd<Object>> = Vec::new();

    //     let collision_count = self.base.get_slide_collision_count();
    //     if collision_count > 0 {
    //         for i in 0..collision_count {
    //             let collision = self.base.get_slide_collision(i);
    //             if let Some(collision) = collision {
    //                 if let Some(object) = collision.get_collider() {
    //                     objects.push(object);
    //                 }
    //             }
    //         }
    //     }

    //     return objects;
    // }

    fn change_direction(&mut self) {
        let pos = match self.platform_standing {
            PlatformDirection::Center => Vector3::new(0.0, 0.0, 0.0),
            PlatformDirection::Right => Vector3::new(4.437, 0.0, 0.0),
            PlatformDirection::Left => Vector3::new(-4.432, 0.0, 0.0),
        };

        let x = pos.x;
        let y = self.base.get_position().y;
        let z = self.base.get_position().z;

        let current_pos = self.base.get_position();
        self.base
            .set_position(Vector3::lerp(current_pos, Vector3::new(x, y, z), 0.3));
    }

    fn death_check(&mut self) {
        if self.dead {
            let mut scene_tree = self.base.get_tree();
            scene_tree.as_deref_mut().unwrap().set_pause(true);

            let parent = self.base.get_parent();
            let ui = parent.as_deref().unwrap().get_node_as::<Control>("UI");
            let mut dead_menu = ui.get_node_as::<Control>("Death");

            dead_menu.set_visible(true);
        }
    }
}

#[godot_api]
impl CharacterBody3DVirtual for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Player {
            speed: 20.0,
            jump_power: 10.0,
            gravity: 30.0,
            dead: false,

            platform_standing: PlatformDirection::Center,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.death_check();

        // mobility
        let mut velocity = self.base.get_velocity();
        let input = Input::singleton();

        // JUMPING
        if !self.base.is_on_floor() {
            velocity.y -= real::from_f64(self.gravity * delta);
        } else {
            self.gravity = 30.0
        }

        if input.is_action_just_pressed("jump".into()) && self.base.is_on_floor() {
            velocity.y += real::from_f64(self.jump_power);
        }

        // ROLL and JUMP CANCEL
        if input.is_action_just_pressed("roll".into()) {
            if self.base.is_on_floor() {
                // IMPL ROLL
            } else {
                self.gravity += 200.0
            }
        }

        // STRAFING
        if input.is_action_just_pressed("left".into()) {
            self.platform_standing = match self.platform_standing {
                PlatformDirection::Center => PlatformDirection::Left,
                PlatformDirection::Right => PlatformDirection::Center,
                PlatformDirection::Left => PlatformDirection::Left,
            };
        }

        if input.is_action_just_pressed("right".into()) {
            self.platform_standing = match self.platform_standing {
                PlatformDirection::Center => PlatformDirection::Right,
                PlatformDirection::Left => PlatformDirection::Center,
                PlatformDirection::Right => PlatformDirection::Right,
            };
        }

        // FORWARD
        velocity.z = -1.0 * real::from_f64(self.speed);

        // apply
        self.base.set_velocity(velocity);
        self.base.move_and_slide();
        self.change_direction();
    }
}
