use bevy::prelude::*;

const G: f32 = 1.0;

const MASSES: [f32; 3] = [1.0, 1.0, 1.0];
const INITIAL_POSITIONS: [Vec2; 3] = [
    Vec2::new(-0.97000436, 0.24308753),
    Vec2::new(0.97000436, -0.24308753),
    Vec2::new(0.0, 0.0),
];
const INITIAL_VELOCITIES: [Vec2; 3] = [
    Vec2::new(0.466203685, 0.43236573),
    Vec2::new(0.466203685, 0.43236573),
    Vec2::new(-0.93240737, -0.86473146),
];

const SCALE: f32 = 300.0;

#[derive(Component, Clone)]
struct Body {
    mass: f32,
    position: Vec2,
    velocity: Vec2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Three Body Problem".to_string(),
                resolution: (800., 800.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, update_bodies)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    for (i, &mass) in MASSES.iter().enumerate() {
        let position = INITIAL_POSITIONS[i] * SCALE;
        let velocity = INITIAL_VELOCITIES[i] * SCALE;

        commands.spawn((
            SpriteBundle {
                image: Sprite {
                    color: Color::hsl(i as f32 * 120.0, 0.8, 0.5),
                    custom_size: Some(Vec2::splat(20.0)),
                    ..default()
                },
                transform: Transform::from_xyz(position.x, position.y, 0.0),
                ..default()
            },
            Body {
                mass,
                position,
                velocity,
            },
        ));
    }
}

fn update_bodies(
    time: Res<Time>,
    mut query: Query<(&mut Body, &mut Transform)>,
) {
    let dt = time.delta_seconds();
    let mut bodies: Vec<(Mut<Body>, Mut<Transform>)> = query.iter_mut().collect();
    let snapshots: Vec<Body> = bodies.iter().map(|(b, _)| b.clone()).collect();
    let accels = calc_accelerations(&snapshots);

    for ((body, transform), accel) in bodies.iter_mut().zip(accels.iter()) {
        // Symplectic Euler integration
        body.velocity += *accel * dt;
        body.position += body.velocity * dt;

        transform.translation = Vec3::new(body.position.x, body.position.y, 0.0);
    }
}

fn calc_accelerations(bodies: &[Body]) -> Vec<Vec2> {
    let mut accelerations = vec![Vec2::ZERO; bodies.len()];

    for i in 0..bodies.len() {
        for j in 0..bodies.len() {
            if i == j {
                continue;
            }

            let dir = bodies[j].position - bodies[i].position;
            let dist_sq = dir.length_squared().max(1e-5); // avoid singularity
            let force_mag = G * bodies[j].mass / dist_sq;
            accelerations[i] += dir.normalize() * force_mag;
        }
    }

    accelerations
}
