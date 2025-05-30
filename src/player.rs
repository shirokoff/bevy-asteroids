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
const DECELERATION_FACTOR: f32 = 2.0;

fn setup(mut commands: Commands, assets_server: ResMut<AssetServer>) {
    commands.spawn((
        (
            Player {
                speed: Vec2 { x: 0.0, y: 0.0 },
            },
            Sprite {
                image: assets_server.load("./PNG/playerShip1_blue.png"),
                ..default()
            },
        ),
        children![(
            Sprite {
                image: assets_server.load("./PNG/Effects/fire01.png"),
                ..default()
            },
            Transform {
                translation: Vec3::new(0.0, -50.0, 0.0),
                ..default()
            },
        )],
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

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        player.speed += forward * THRUST_SPEED * time.delta_secs();
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        let linear_speed = player.speed.length();
        let new_linear_speed = linear_speed - THRUST_SPEED * time.delta_secs();
        player.speed = player.speed.normalize_or_zero() * new_linear_speed.max(0.0);
    } else {
        let linear_speed = player.speed.length();
        let new_linear_speed = linear_speed - DECELERATION_FACTOR * time.delta_secs();
        player.speed = player.speed.normalize_or_zero() * new_linear_speed.max(0.0);
    }

    transform.rotate_z(rotation);
    transform.translation += player.speed.extend(0.0);
}
