use benimator::AnimationPlugin;
use bevy::prelude::*;
// use bevy_inspector_egui::WorldInspectorPlugin;
use doup_fortuna::FortunaPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(WindowDescriptor {
            title: "Fortuna".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AnimationPlugin::default())
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(FortunaPlugin)
        // .add_plugin(bevy_framepace::FramepacePlugin {
        //     enabled: true,
        //     framerate_limit: bevy_framepace::FramerateLimit::Manual(15),
        //     warn_on_frame_drop: true,
        //     safety_margin: std::time::Duration::from_micros(100),
        //     power_saver: bevy_framepace::PowerSaver::Disabled,
        // })
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
