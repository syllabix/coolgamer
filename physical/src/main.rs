use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup_graphics, setup_physics))
        // .add_systems(Update, movement)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn setup_physics(mut commands: Commands) {
    commands.spawn((
        Collider::cuboid(500., 50.),
        Transform::from_xyz(0., -100., 0.),
    ));

    // Spawn a ball sprite
    commands
        .spawn((
            RigidBody::Dynamic,
            Sprite {
                color: Color::srgb(0.5, 0.5, 1.0),
                custom_size: Some(Vec2::new(100.0, 100.0)),

                ..default()
            },
        ))
        .insert(Collider::ball(50.))
        .insert(Restitution::coefficient(0.99))
        .insert(Transform::from_xyz(0.0, 400., 0.));
}

// fn movement(positions: Query<&Transform, With<RigidBody>>) {
//     for transform in positions.iter() {
//         println!("altitude: {}", transform.translation.y)
//     }
// }