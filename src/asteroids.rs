use bevy::prelude::*;

const ASTEROID_SPEED: f32 = 50.0;
const ASTEROID_SPAWN_INTERVAL: f32 = 10.0; // Cooldown period in seconds
const NUMBER_OF_ASTEROIDS_TO_SPAWN: usize = 5;

#[derive(Component)]
pub struct Asteroid {
    pub speed: Vec2,
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
