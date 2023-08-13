use godot::prelude::*;

mod main_scene;
mod player;
mod hud;
mod portal;

struct RetroRunning;

#[gdextension]
unsafe impl ExtensionLibrary for RetroRunning {}
