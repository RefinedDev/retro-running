use godot::{
    engine::{Control, ControlVirtual, Label},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct UI {
    #[base]
    base: Base<Control>,
}

impl UI {
    pub fn update_score(&mut self, score: i32) {
        let mut label = self.base.get_node_as::<Label>("Score");
        label.set_text(score.to_string().into());
    }
}

#[godot_api]
impl ControlVirtual for UI {
    fn init(base: Base<Control>) -> Self {
        UI { base }
    }
}
