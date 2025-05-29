use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct SpaceWrapPlugin;

impl Plugin for SpaceWrapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, space_wrap);
    }
}

fn space_wrap(
    mut query: Query<&mut Transform, With<Sprite>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let width = window.width() / 2.0;
    let height = window.height() / 2.0;

    for mut transform in query.iter_mut() {
        if transform.translation.x > width {
            transform.translation.x = -width;
        } else if transform.translation.x < -width {
            transform.translation.x = width;
        }

        if transform.translation.y > height {
            transform.translation.y = -height;
        } else if transform.translation.y < -height {
            transform.translation.y = height;
        }
    }
}
