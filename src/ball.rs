use bevy::{
    prelude::*,
};

use crate::map;

const BALL_SIZE_X: f32 = 17.0;
const BALL_SIZE_Y: f32 = 17.0;

pub struct Size {
    x: f32,
    y: f32,
}

pub struct Ball {
    speed: f32,
}

enum Collider {
    Wall,
    Ball,
}

pub fn ball_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("textures/ball.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(4.0, 4.0), 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..Default::default()
        })
        .insert(Ball { speed: 400.0 })
        .insert(Size { x: BALL_SIZE_X, y: BALL_SIZE_Y} )
        .insert(Collider::Ball);

    // Add walls
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(map::X_BOUND, map::Y_BOUND);

    // left
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(-bounds.x.clone() / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y.clone() + wall_thickness.clone())),
            ..Default::default()
        })
        .insert(Collider::Wall);
    // right
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(bounds.x.clone() / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(wall_thickness.clone(), bounds.y.clone() + wall_thickness.clone())),
            ..Default::default()
        })
        .insert(Collider::Wall);
    // bottom
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(0.0, -bounds.y.clone() / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(bounds.x.clone() + wall_thickness.clone(), wall_thickness.clone())),
            ..Default::default()
        })
        .insert(Collider::Wall);
    // top
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material,
            transform: Transform::from_xyz(0.0, bounds.y.clone() / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(bounds.x.clone() + wall_thickness.clone(), wall_thickness.clone())),
            ..Default::default()
        })
        .insert(Collider::Wall);
}

pub fn ball_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Ball, &Size, &mut Transform)>,
) {
    if let Ok((ball, size, mut transform)) = query.single_mut() {
        let mut x_direction = 0.0;
        let mut y_direction = 0.0;

        if keyboard_input.pressed(KeyCode::Left) {
            x_direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            x_direction += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            y_direction += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            y_direction -= 1.0;
        }

        let translation = &mut transform.translation;
        translation.x += time.delta_seconds() * x_direction * ball.speed;
        translation.y += time.delta_seconds() * y_direction * ball.speed;
        translation.x = translation.x.min(map::X_BOUND / 2. - size.x).max(-map::X_BOUND / 2. + size.x);
        translation.y = translation.y.min(map::Y_BOUND / 2. - size.y).max(-map::Y_BOUND / 2. + size.y);
    }
}
