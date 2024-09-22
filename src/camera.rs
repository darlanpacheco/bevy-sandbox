use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    window::WindowResized,
};

#[derive(Component)]
struct Canvas;
#[derive(Component)]
struct InGameCamera;
#[derive(Component)]
pub struct OuterCamera;

const RES_WIDTH: u32 = 320;
const RES_HEIGHT: u32 = 180;

pub const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);
const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

pub fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let canvas_size = Extent3d {
        width: RES_WIDTH,
        height: RES_HEIGHT,
        ..default()
    };
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    canvas.resize(canvas_size);
    let image_handle = images.add(canvas);
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        },
        InGameCamera,
        PIXEL_PERFECT_LAYERS,
    ));
    commands.spawn((
        SpriteBundle {
            texture: image_handle,
            ..default()
        },
        Canvas,
        HIGH_RES_LAYERS,
    ));
    commands.spawn((Camera2dBundle::default(), OuterCamera, HIGH_RES_LAYERS));
}

pub fn fit_canvas(
    mut resize_events: EventReader<WindowResized>,
    mut projections: Query<&mut OrthographicProjection, With<OuterCamera>>,
) {
    for event in resize_events.read() {
        let h_scale = event.width / RES_WIDTH as f32;
        let v_scale = event.height / RES_HEIGHT as f32;
        let mut projection = projections.single_mut();
        projection.scale = 1. / h_scale.min(v_scale).round();
    }
}
