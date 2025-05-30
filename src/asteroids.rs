use crate::bullets::Bullet;
use bevy::prelude::*;

const ASTEROID_SPEED: f32 = 50.0;
const ASTEROID_SPAWN_INTERVAL: f32 = 10.0; // Cooldown period in seconds
const NUMBER_OF_ASTEROIDS_TO_SPAWN: usize = 5;

#[derive(PartialEq)]
enum AsteroidSize {
    Small,
    // Medium,
    Large,
}

#[derive(Component)]
pub struct Asteroid {
    speed: Vec2,
    collision_radius: f32,
    size: AsteroidSize,
}

#[derive(Resource)]
struct AsteroidSpawnTimer(Timer);

pub struct AsteroidsPlugin;
impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AsteroidSpawnTimer(Timer::from_seconds(
            ASTEROID_SPAWN_INTERVAL,
            TimerMode::Repeating,
        )))
        .add_systems(Update, bullet_collision)
        .add_systems(Update, spawn_asteroids)
        .add_systems(Update, asteroid_movement);
    }
}

fn spawn_asteroids(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<AsteroidSpawnTimer>,
) {
    timer.0.tick(time.delta());
    if !timer.0.finished() {
        return;
    }

    for _ in 0..NUMBER_OF_ASTEROIDS_TO_SPAWN {
        commands.spawn((
            Asteroid {
                speed: Vec2::from_angle(rand::random::<f32>() * std::f32::consts::PI * 2.0)
                    * ASTEROID_SPEED,
                collision_radius: 50.0,
                size: AsteroidSize::Large,
            },
            Sprite {
                image: assets_server.load("./PNG/Meteors/meteorBrown_big1.png"),
                ..default()
            },
            Transform {
                translation: Vec3::new(
                    rand::random::<f32>() * 800.0 - 400.0,
                    rand::random::<f32>() * 600.0 - 300.0,
                    0.0,
                ),
                rotation: Quat::from_rotation_z(rand::random::<f32>() * std::f32::consts::PI * 2.0),
                ..default()
            },
        ));
    }
}

fn asteroid_movement(time: Res<Time>, mut query: Query<(&mut Transform, &Asteroid)>) {
    for (mut transform, asteroid) in query.iter_mut() {
        transform.translation += asteroid.speed.extend(0.0) * time.delta_secs();
        transform.rotate_z(std::f32::consts::PI * time.delta_secs());
    }
}

fn bullet_collision(
    mut commands: Commands,
    mut asteroids: Query<(Entity, &Transform, &Asteroid)>,
    bullets: Query<&Transform, With<Bullet>>,
    assets_server: Res<AssetServer>,
) {
    for bullet_transform in bullets.iter() {
        for (asteroid_entity, asteroid_transform, asteroid) in asteroids.iter_mut() {
            if bullet_transform
                .translation
                .distance(asteroid_transform.translation)
                < asteroid.collision_radius
            {
                if asteroid.size == AsteroidSize::Large {
                    for _ in 0..NUMBER_OF_ASTEROIDS_TO_SPAWN {
                        commands.spawn((
                            Asteroid {
                                speed: Vec2::from_angle(
                                    rand::random::<f32>() * std::f32::consts::PI * 2.0,
                                ) * (ASTEROID_SPEED * 1.5),
                                collision_radius: 15.0,
                                size: AsteroidSize::Small,
                            },
                            Sprite {
                                image: assets_server.load("./PNG/Meteors/meteorBrown_small1.png"),
                                ..default()
                            },
                            Transform {
                                translation: asteroid_transform.translation,
                                rotation: Quat::from_rotation_z(
                                    rand::random::<f32>() * std::f32::consts::PI * 2.0,
                                ),
                                ..default()
                            },
                        ));
                    }
                }

                commands.entity(bullet_entity).try_despawn();
                commands.entity(asteroid_entity).try_despawn();
            }
        }
    }
}
