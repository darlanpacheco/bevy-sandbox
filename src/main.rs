use crate::camera::*;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
mod camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(Msaa::Off)
        .add_systems(Startup, (setup_camera, setup_scene))
        .add_systems(Update, (fit_canvas, move_player))
        .run();
}

#[derive(Component)]
struct Player;

const PLAYER_SPEED: f32 = 100.0;
fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };
    let mut direction = Vec2::ZERO;
    if kb_input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }
    if kb_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }
    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }
    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }
    let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_seconds();
    player.translation += move_delta.extend(0.);
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(32.0)).into(),
            material: materials.add(Color::BLACK),
            ..default()
        },
        PIXEL_PERFECT_LAYERS,
        Player,
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(32.0)).into(),
            transform: Transform::from_xyz(100.0, 0.0, 0.0),
            material: materials.add(Color::BLACK),
            ..default()
        },
        PIXEL_PERFECT_LAYERS,
    ));
}
