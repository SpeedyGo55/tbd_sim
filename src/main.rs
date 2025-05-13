// three body problem simulation
use bevy::prelude::*;

#[derive(Component)]
struct Body {
    mass: f32,
    position: Vec2,
    velocity: Vec2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update_bodies)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2d);

    let bodies = [
        meshes.add(Circle::new(20.0)),
        meshes.add(Circle::new(20.0)),
        meshes.add(Circle::new(20.0)),
    ];

    commands.spawn((
        Mesh2d(bodies[0].clone()),
        MeshMaterial2d(materials.add(Color::hsl(0.0, 1.0, 0.5))),
        Transform::from_xyz(-30.0, 0.0, 0.0),
        Body {
            mass: 1.0,
            position: Vec2::new(-30.0, 0.0),
            velocity: Vec2::new(0.0, 1.0),
        },
    ));

    commands.spawn((
        Mesh2d(bodies[1].clone()),
        MeshMaterial2d(materials.add(Color::hsl(120.0, 1.0, 0.5))),
        Transform::from_xyz(30.0, 0.0, 0.0),
        Body {
            mass: 1.0,
            position: Vec2::new(30.0, 0.0),
            velocity: Vec2::new(1.0, 1.0),
        },
    ));

    commands.spawn((
        Mesh2d(bodies[2].clone()),
        MeshMaterial2d(materials.add(Color::hsl(240.0, 1.0, 0.5))),
        Transform::from_xyz(0.0, 30.0, 0.0),
        Body {
            mass: 1.0,
            position: Vec2::new(0.0, 30.0),
            velocity: Vec2::new(1.0, 0.0),
        },
    ));
}


fn update_bodies(
    time: Res<Time>,
    mut query: Query<(&mut Body, &mut Transform)>,
) {
    let dt = time.delta_secs();

    let mut bodies: Vec<(Mut<Body>,Mut<Transform>)> = query.iter_mut().collect();
    let (body1, transform1) = bodies[0];
    let (body2, transform2) = bodies[1];
    let (body3, transform3) = bodies[2];

    calc_new_velocity(&mut body1, &mut body2, &mut body3);

    transform1.translation += Vec3::new(body1.velocity.x * dt, body1.velocity.y * dt, 0.0);
    transform2.translation += Vec3::new(body2.velocity.x * dt, body2.velocity.y * dt, 0.0);
    transform3.translation += Vec3::new(body3.velocity.x * dt, body3.velocity.y * dt, 0.0);
    body1.position += body1.velocity * dt;
    body2.position += body2.velocity * dt;
    body3.position += body3.velocity * dt;
}

fn calc_new_velocity(body1: &mut Body, body2: &mut Body, body3: &mut Body) {
    let G = 6.67430e-11; // gravitational constant
    let center_of_mass12 = (body1.mass * body1.position + body2.mass * body2.position) / (body1.mass + body2.mass);
    let center_of_mass13 = (body1.mass * body1.position + body3.mass * body3.position) / (body1.mass + body3.mass);
    let center_of_mass23 = (body2.mass * body2.position + body3.mass * body3.position) / (body2.mass + body3.mass);

    let distance1_to_com23 = (body1.position - center_of_mass23).length();
    let distance2_to_com13 = (body2.position - center_of_mass13).length();
    let distance3_to_com12 = (body3.position - center_of_mass12).length();

    let mass12 = body1.mass + body2.mass;
    let mass13 = body1.mass + body3.mass;
    let mass23 = body2.mass + body3.mass;

    let force1 = G * body1.mass * mass23 / (distance1_to_com23 * distance1_to_com23);
    let force2 = G * body2.mass * mass13 / (distance2_to_com13 * distance2_to_com13);
    let force3 = G * body3.mass * mass12 / (distance3_to_com12 * distance3_to_com12);
    let acceleration1 = force1 / body1.mass;
    let acceleration2 = force2 / body2.mass;
    let acceleration3 = force3 / body3.mass;
    let direction1 = (body1.position - center_of_mass23).normalize();
    let direction2 = (body2.position - center_of_mass13).normalize();
    let direction3 = (body3.position - center_of_mass12).normalize();
    body1.velocity += direction1 * acceleration1;
    body2.velocity += direction2 * acceleration2;
    body3.velocity += direction3 * acceleration3;

}