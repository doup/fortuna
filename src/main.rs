use bevy::prelude::*;
use doup_fortuna::FortunaPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Fortuna".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FortunaPlugin)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}
