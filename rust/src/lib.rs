use godot::prelude::*;

mod main_scene;
mod player;
mod hud;
struct RetroRunning;

#[gdextension]
unsafe impl ExtensionLibrary for RetroRunning {}
