use godot::{
    engine::{CharacterBody3D, CharacterBody3DVirtual},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player {
    pub speed: f64,
    jump_power: f64,
    gravity: f64,

    platform_standing: char,

    #[base]
    base: Base<CharacterBody3D>,
}

impl Player {
    // pub fn get_platform_standing_on(&mut self) -> Option<Gd<Node>> {
    //     let collisions_no = self.base.get_slide_collision_count();
    //     if collisions_no > 0 {
    //         for i in 0..=collisions_no {
    //             let collision = self.base.get_slide_collision(i);

    //             if let Some(collision_obj) = collision {
    //                 if let Some(object) = collision_obj.get_collider() {
    //                     let collider = object.cast::<Node>();

    //                     if collider.is_in_group("floor".into()) {
    //                         return Some(collider);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return None;
    // }

    pub fn change_direction(&mut self, direction: &str) {
        // let platform = self.get_platform_standing_on();
        // if let Some(platform) = platform {
        //     let platform = platform.get_node_as::<MeshInstance3D>(direction);

        //     let x = platform.get_position().x;
        //     let y = self.base.get_position().y;
        //     let z = self.base.get_position().z;

        //     let current_pos = self.base.get_position();
        //     self.base.set_position(Vector3::lerp(current_pos, Vector3::new(x,y,z), 0.3));
        // }

        // ALT: hardcoding positions, the advantage here is you can change position IN AIR unlike the code above
        let center = Vector3::new(0.0, 0.0, 0.0);
        let right = Vector3::new(4.437, 0.0, 0.0);
        let left = Vector3::new(-4.432, 0.0, 0.0);
        let pos = match direction {
            "Center" => center,
            "Right" => right,
            "Left" => left,
            _ => center,
        };

        let x = pos.x;
        let y = self.base.get_position().y;
        let z = self.base.get_position().z;

        let current_pos = self.base.get_position();
        self.base
            .set_position(Vector3::lerp(current_pos, Vector3::new(x, y, z), 0.3));
    }
}

#[godot_api]
impl CharacterBody3DVirtual for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Player {
            speed: 20.0,
            jump_power: 10.0,
            gravity: 30.0,

            platform_standing: 'C', // 'C' = Center, 'L' = Left, 'R' = Right
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
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
                self.gravity += 500.0
            }
        }

        // STRAFING CODE
        if input.is_action_just_pressed("left".into()) {
            self.platform_standing = match self.platform_standing {
                'C' => 'L',
                'R' => 'C',
                _ => 'L'
            };
        }

        if input.is_action_just_pressed("right".into()) {
            self.platform_standing = match self.platform_standing {
                'C' => 'R',
                'L' => 'C',
                _ => 'R'
            };
        }

        match self.platform_standing {
            'C' => self.change_direction("Center"),
            'R' => self.change_direction("Right"),
            'L' => self.change_direction("Left"),
            _ => {}
        }

        // GOING FORWARD
        velocity.z = -1.0 * real::from_f64(self.speed);

        self.base.set_velocity(velocity);
        self.base.move_and_slide();
    }
}