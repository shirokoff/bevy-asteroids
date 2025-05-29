use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: Vec2,
}

const ROTATION_SPEED: f32 = std::f32::consts::PI;
const THRUST_SPEED: f32 = 10.0;
const DECELERATION_FACTOR: f32 = 0.98;

fn setup(mut commands: Commands, assets_server: ResMut<AssetServer>) {
    commands.spawn((
        Player {
            speed: Vec2 { x: 0.0, y: 0.0 },
        },
        Sprite {
            image: assets_server.load("./PNG/playerShip1_blue.png"),
            ..default()
        },
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_ship: Single<(&mut Player, &mut Transform)>,
    time: Res<Time>,
) {
    let (mut player, mut transform) = player_ship.into_inner();
    let forward = transform.up().truncate();
    let mut rotation = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        rotation += ROTATION_SPEED * time.delta_secs();
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        rotation -= ROTATION_SPEED * time.delta_secs();
    }

    transform.rotate_z(rotation);

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        player.speed += forward * THRUST_SPEED * time.delta_secs();
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        player.speed -= forward * THRUST_SPEED * time.delta_secs();
    }

    player.speed *= DECELERATION_FACTOR;

    transform.translation += player.speed.extend(0.0);
}
