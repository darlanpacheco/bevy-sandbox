use crate::camera::*;
use crate::player::*;
use avian2d::collision::*;
use avian2d::math::Vector;
use avian2d::prelude::*;
use bevy::prelude::*;
mod camera;
mod player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default().with_length_unit(75.0),
        ))
        .insert_resource(Msaa::Off)
        .insert_resource(Gravity(Vector::new(0.0, 0.0)))
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Player,
            SpriteBundle {
                texture: asset_server.load("mario.png"),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::rectangle(15.0, 28.0),
        ))
        .with_children(|parent| {
            parent.spawn(main_camera());
        });
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pipe.png"),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(48.0, 16.0),
    ));

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(24.0),
                height: Val::Percent(8.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ButtonBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            });
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                image: UiImage::new(asset_server.load("pipe.png")),
                ..default()
            });
        });
}
