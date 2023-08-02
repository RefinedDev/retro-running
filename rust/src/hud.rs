use godot::{
    engine::{notify::ControlNotification, Control, ControlVirtual, Label, Engine},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct UI {
    pub paused: bool,

    fps_label: Option<Gd<Label>>,
    score_label: Option<Gd<Label>>,

    #[base]
    base: Base<Control>,
}

#[godot_api]
impl UI {
    pub fn update_score(&mut self, score: i32) {
        let score_label = self.score_label.as_deref_mut().unwrap();
        score_label.set_text(score.to_string().into());
    }

    pub fn pause(&mut self) {
        if self.paused {
            return;
        }

        let mut scene_tree = self.base.get_tree();
        let scene_tree = scene_tree.as_deref_mut().unwrap();

        let mut pause_menu = self.base.get_node_as::<Control>("Pause Menu");

        self.paused = true;
        pause_menu.set_visible(true);
        scene_tree.set_pause(true);
    }

    #[func]
    pub fn unpause(&mut self) {
        let mut st = self.base.get_tree();
        let mut pause_menu = self.base.get_node_as::<Control>("Pause Menu");

        pause_menu.set_visible(false);
        st.as_deref_mut().unwrap().set_pause(false);
        self.paused = false;
    }

    #[func]
    fn exit_game(&mut self) {
        let mut st = self.base.get_tree();
        st.as_deref_mut().unwrap().quit();
    }
}

#[godot_api]
impl ControlVirtual for UI {
    fn init(base: Base<Control>) -> Self {
        UI {
            paused: false,
            fps_label: None,
            score_label: None,
            base,
        }
    }

    fn ready(&mut self) {
        self.fps_label = Some(self.base.get_node_as::<Label>("FPS"));
        self.score_label = Some(self.base.get_node_as::<Label>("Score"));
    }
    
    fn process(&mut self, _:f64) {
        let fps = Engine::singleton().get_frames_per_second().to_string().into();
        self.fps_label.as_deref_mut().unwrap().set_text(fps);

        let input = Input::singleton();
        if input.is_action_just_pressed("pause".into()) {
            if self.paused {
                self.unpause();
            } else {
                self.pause();
            }
        }
    }

    fn on_notification(&mut self, what: ControlNotification) {
        if Engine::singleton().is_editor_hint() {
            return
        }
        
        if let ControlNotification::WmWindowFocusOut = what {
            self.pause(); // pause the game automatically if player goes out of game
        }
    }
}
