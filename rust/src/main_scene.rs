use crate::hud::UI;
use crate::player::Player;
use crate::portal::Portal;
use crate::portal::Realm;

use godot::{
    engine::{MeshInstance3D, StaticBody3D, Timer},
    prelude::*,
};

use rand::distributions::Standard;
use rand::prelude::Distribution;

pub enum PlatformDirection {
    Left,
    Right,
    Center,
}

impl ToString for PlatformDirection {
    fn to_string(&self) -> String {
        match self {
            PlatformDirection::Left => "Left".to_string(),
            PlatformDirection::Right => "Right".to_string(),
            PlatformDirection::Center => "Center".to_string(),
        }
    }
}
impl Distribution<PlatformDirection> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> PlatformDirection {
        match rng.gen_range(0..=2) {
            0 => PlatformDirection::Left,
            1 => PlatformDirection::Center,
            2 => PlatformDirection::Right,
            _ => PlatformDirection::Center,
        }
    }
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    score: i32,
    game_intensity: i8, // how fast is game going

    floor_scene: Gd<PackedScene>,
    portal_scene: Gd<PackedScene>,

    current_realm: Realm,

    floor_count: f32,
    current_floor: Option<Gd<StaticBody3D>>,

    #[base]
    base: Base<Node>,
}

#[godot_api]
impl Main {
    #[func]
    fn spawn_floor(&mut self) {
        let floor = self.floor_scene.instantiate_as::<StaticBody3D>();
        self.base.add_child(floor.share().upcast());

        self.current_floor = Some(floor);

        self.floor_count += 1.0;
    }

    #[func]
    fn spawn_portal(&mut self) {
        let direction: PlatformDirection = rand::random();

        let mut portal_struct = self
            .portal_scene
            .instantiate_as::<Node3D>()
            .cast::<Portal>();
        portal_struct.bind_mut().setup(&self.current_realm);

        let portal = portal_struct.upcast::<Node3D>();

        let current_floor = self.current_floor.as_deref().unwrap();
        let mut direction_platform =
            current_floor.get_node_as::<MeshInstance3D>(direction.to_string());

        direction_platform.add_child(portal.share().upcast());
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
            if i.1 == self.score && self.game_intensity == i.0 as i8 {
                // check if score equal and the intensity is also equal to the respective index
                self.game_intensity += 1;
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
            game_intensity: 0,

            floor_scene: PackedScene::new(),
            portal_scene: PackedScene::new(),

            current_realm: Realm::OVERWORLD,

            floor_count: 0.0,
            current_floor: None,

            base,
        }
    }

    fn ready(&mut self) {
        let mut scene_tree = self.base.get_tree();
        let scene_tree = scene_tree.as_deref_mut().unwrap();

        scene_tree.set_pause(true);

        self.floor_scene = load("res://Scenes/floor.tscn");
        self.portal_scene = load("res://Scenes//portal.tscn")
    }

    fn process(&mut self, _: f64) {
        match self.current_floor.as_mut() {
            // lerping new floors just for fun
            Some(floor) => {
                let current_pos = floor.get_position();
                let target_pos = Vector3::new(0.0, 1.0, -70.0 * self.floor_count);
                floor.set_position(Vector3::lerp(current_pos, target_pos, 0.3));

                if floor.get_position() == target_pos {
                    self.current_floor = None
                }
            }
            None => {}
        }
    }
}
