use bevy::prelude::*;

mod bullets;
mod player;
mod space_wrap;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(space_wrap::SpaceWrapPlugin)
        .add_plugins(bullets::BulletsPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
