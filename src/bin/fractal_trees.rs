use std::f32::consts::PI;

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        //.add_system(sprite_movement)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    branch(commands, 100.);
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(mut sprite_position: Query<&mut Transform>) {
    for mut transform in &mut sprite_position {
        transform.rotate_z(f32::to_radians(1.));
    }
}

fn branch(mut commands: Commands, len: f32) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(1., len)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::from((-35., 185., 0.)),
            rotation: Quat::from_rotation_z(PI / 4.),
            scale: Vec3::new(1., 1., 1.),
        },
        ..default()
    });

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(1., len)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::from((0., 100., 0.)),
            rotation: Quat::from_rotation_z(0.),
            scale: Vec3::new(1., 1., 1.),
        },
        ..default()
    });
}
