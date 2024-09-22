use bevy::prelude::Camera2dBundle;
use bevy::render::camera::ScalingMode;

pub fn main_camera() -> Camera2dBundle {
    let mut main_camera = Camera2dBundle::default();
    main_camera.projection.scaling_mode = ScalingMode::FixedHorizontal(320.0);
    main_camera.projection.scaling_mode = ScalingMode::FixedVertical(180.0);
    return main_camera;
}
