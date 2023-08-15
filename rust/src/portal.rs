use godot::{
    engine::{base_material_3d::Feature, MeshInstance3D, StandardMaterial3D, StaticBody3DVirtual},
    prelude::*,
};

use rand::{distributions::Standard, prelude::Distribution};

// REALM ENUM AND IMPLEMENTATIONS
#[derive(PartialEq, Eq)]
pub enum Realm {
    Overworld,
    Nether,
    EightBit,
    Inverted,
}

impl Distribution<Realm> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Realm {
        match rng.gen_range(0..=3) {
            0 => Realm::Overworld,
            1 => Realm::Nether,
            2 => Realm::EightBit,
            3 => Realm::Inverted,
            _ => Realm::Overworld,
        }
    }
}

impl Realm {
    fn get_colour(&self) -> (u8, u8, u8) {
        match self {
            Self::Overworld => (77, 140, 87),
            Self::EightBit => (255, 105, 180),
            Self::Nether => (139, 0, 0),
            Self::Inverted => (104, 51, 255),
        }
    }
}

// GAME OBJECT PORTAL
#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Portal {
    portal_realm: Realm,

    #[base]
    base: Base<Node3D>,
}

#[godot_api]
impl Portal {
    pub fn setup(&mut self, current_realm: &Realm) {
        let mut random_realm: Realm = rand::random();

        while &random_realm == current_realm {
            // we dont want to have the portal of the same realm (eg. overworld portal in overworld realm)
            random_realm = rand::random();
        }

        self.portal_realm = random_realm;
        self.change_texture();
    }

    fn change_texture(&mut self) {
        let mut portal = self.base.get_node_as::<MeshInstance3D>("portal");

        let rgba = self.portal_realm.get_colour();
        let colour = Color::from_rgba8(rgba.0, rgba.1, rgba.2, 255);

        let mut material = StandardMaterial3D::new();
        material.set_albedo(colour);
        material.set_emission(colour);
        material.set_feature(Feature::FEATURE_EMISSION, true);

        portal.set_surface_override_material(0, material.upcast());
    }

    #[func]
    fn body_entered(&mut self, body: Gd<Node3D>) {
        if body.is_in_group("player".into()) {
            godot_print!("warp!")
        }
    }
}

#[godot_api]
impl StaticBody3DVirtual for Portal {
    fn init(base: Base<Node3D>) -> Self {
        Portal {
            portal_realm: Realm::Overworld,

            base,
        }
    }
}
