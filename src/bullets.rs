use crate::player::Player;
use bevy::prelude::*;

const BULLET_LINER_SPEED: f32 = 600.0;
const BULLET_LIFETIME: f32 = 3.0;
const FIRE_COOLDOWN: f32 = 0.5;

#[derive(Resource)]
struct FireRateTimer(Timer);

#[derive(Component)]
struct Bullet {
    speed: Vec2,
}

#[derive(Component)]
struct BulletLifetime {
    timer: Timer,
}

pub struct BulletsPlugin;
impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FireRateTimer(Timer::from_seconds(
            FIRE_COOLDOWN,
            TimerMode::Once,
        )))
        .add_systems(Update, bullet_movement)
        .add_systems(Update, bullet_despawn)
        .add_systems(Update, bullet_spawn);
    }
}

fn bullet_spawn(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Single<&Transform, With<Player>>,
    assets_server: Res<AssetServer>,
    time: Res<Time>,
    mut fire_rate_timer: ResMut<FireRateTimer>,
) {
    fire_rate_timer.0.tick(time.delta());

    if !keyboard_input.just_pressed(KeyCode::Space) || !fire_rate_timer.0.finished() {
        return;
    }

    let player_transform = player.into_inner();

    commands.spawn((
        Bullet {
            speed: player_transform.up().truncate() * BULLET_LINER_SPEED,
        },
        BulletLifetime {
            timer: Timer::from_seconds(BULLET_LIFETIME, TimerMode::Once),
        },
        Transform {
            translation: player_transform.translation,
            rotation: player_transform.rotation,
            ..default()
        },
        Sprite {
            image: assets_server.load("./PNG/Lasers/laserBlue01.png"),
            ..default()
        },
    ));

    fire_rate_timer.0.reset();
}

fn bullet_movement(time: Res<Time>, mut query: Query<(&mut Transform, &Bullet)>) {
    for (mut transform, bullet) in query.iter_mut() {
        transform.translation += bullet.speed.extend(0.0) * time.delta_secs();
    }
}

fn bullet_despawn(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut BulletLifetime)>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
