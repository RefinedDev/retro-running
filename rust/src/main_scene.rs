use crate::hud::UI;
use crate::player::Player;

use godot::{
    engine::{StaticBody3D, Timer},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    score: i32,
    player_intensity: i8, // how fast the player is running

    floor_scene: Gd<PackedScene>,
    floor_count: f32,
    floor_to_lerp: Option<Gd<StaticBody3D>>,
    
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl Main {
    #[func]
    fn spawnfloor_timeout(&mut self) {
        let floor = self.floor_scene.instantiate_as::<StaticBody3D>();
        self.base.add_child(floor.share().upcast());

        self.floor_to_lerp = Some(floor);

        self.floor_count += 1.0;
    }

    #[func]
    fn increase_score(&mut self) {
        let mut ui = self.base.get_node_as::<UI>("UI");
        self.score += 1;
        ui.bind_mut().update_score(self.score);

        let scores_required_to_increase_game_speed: [i32; 8] =
            [100, 200, 300, 400, 500, 600, 700, 850];
        // player's speed increased when they hit this score

        for i in scores_required_to_increase_game_speed
            .into_iter()
            .enumerate()
        {
            if i.1 == self.score && self.player_intensity == i.0 as i8 {
                // check if score equal and the intensity is also equal to the respective index
                self.player_intensity += 1;
                self.increase_speed();
            }
        }
    }

    fn increase_speed(&mut self) {
        const TIME_DECEASE: f64 = 0.1;
        const SPEED_INCREASE: f64 = 10.0;

        let mut player = self.base.get_node_as::<Player>("Player");
        let mut player = player.bind_mut();
        let mut spawn_floor_timeout = self.base.get_node_as::<Timer>("Spawn_floor");

        let current_wait_time = spawn_floor_timeout.get_wait_time();
        let current_plr_speed = player.speed;

        let new_speed = current_plr_speed + SPEED_INCREASE;

        player.speed = new_speed;
        spawn_floor_timeout.set_wait_time(current_wait_time - TIME_DECEASE);

        godot_print!("New Player Speed: {}", player.speed);
        godot_print!(
            "New Floor Generation Speed: {}",
            spawn_floor_timeout.get_wait_time()
        );
    }
}

#[godot_api]
impl NodeVirtual for Main {
    fn init(base: Base<Node>) -> Self {
        Main {
            score: 0,
            player_intensity: 0,

            floor_scene: PackedScene::new(),
            floor_count: 0.0,
            floor_to_lerp: None,
            base,
        }
    }

    fn ready(&mut self) {
        self.floor_scene = load("res://Scenes/floor.tscn");
    }

    fn process(&mut self, _: f64) {
        match self.floor_to_lerp.as_mut() {
            // lerping new floors just for fun
            Some(floor) => {
                let current_pos = floor.get_position();
                let target_pos = Vector3::new(0.0, 1.0, -70.0 * self.floor_count);
                floor.set_position(Vector3::lerp(current_pos, target_pos, 0.3));

                if floor.get_position() == target_pos {
                    self.floor_to_lerp = None
                }
            }
            None => {}
        }
    }
}
