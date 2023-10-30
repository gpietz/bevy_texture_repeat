mod camera;
mod assets;

use std::borrow::BorrowMut;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy::render::render_resource::{AddressMode, TextureDescriptor};

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    let height = 900.0;
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: (height * RESOLUTION),
            height,
            title: "Bevy Texture Demo".to_string(),
            present_mode: bevy::window::PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_event::<AssetEvent<Image>>()
        .add_startup_system(spawn_camera)
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_system(adjust_textures)
        .run();
}

fn debug_asset_events(mut asset_event: EventReader<AssetEvent<Image>>) {
    for ev in asset_event.iter() {
        println!("Ok - debug_asset_events");
    }
}

fn spawn_camera(mut commands: Commands, mut windows: ResMut<Windows>) {
    let mut camera = OrthographicCameraBundle::new_2d();
    // camera.orthographic_projection.top = 0.0;
    // camera.orthographic_projection.bottom = 1.0;
    // camera.orthographic_projection.left = -1.0 * RESOLUTION;
    // camera.orthographic_projection.right = 1.0 * RESOLUTION;
    // camera.orthographic_projection.scaling_mode = ScalingMode::WindowSize;
    commands.spawn_bundle(camera);
}

fn setup(mut commands: Commands, window: ResMut<Windows>, asset_server: Res<AssetServer>) {
    println!("Setup1");

    let window_width = window.get_primary().unwrap().width();
    let mut texture_handle: Handle<Image>= asset_server.load("wall_horizontal_01.png");

    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 900.0),
            ..Default::default()
        },
        sprite: Sprite {
            custom_size: Some(Vec2::new(window_width, 30.0)),
            color: Color::WHITE,
            ..Default::default()
        },
        texture: texture_handle,
        ..Default::default()
    }).insert(Name::new("player"));
}

fn adjust_textures (mut images: ResMut<Assets<Image>>) {
    if let Some(my_image) = images.get_mut("wall_horizontal_01.png") {
        println!("Adjusting texture");
        my_image.sampler_descriptor.address_mode_u = AddressMode::Repeat;
        my_image.sampler_descriptor.address_mode_v = AddressMode::Repeat;
        my_image.sampler_descriptor.address_mode_w = AddressMode::Repeat;
    }
}