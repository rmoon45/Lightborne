use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
        view::RenderLayers,
    },
};

use crate::{camera::MainCamera, light::segments::LightSegmentMarker};

pub struct LightingPlugin;
impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LightingAssets::default())
            .add_systems(Startup, setup)
            .add_systems(PostUpdate, update)
            .add_systems(PostUpdate, draw_beams);
    }
}
#[derive(Component)]
pub struct ImageMarker;

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut lighting_assets: ResMut<LightingAssets>,
    asset_server: Res<AssetServer>,
) {
    let size = Extent3d {
        width: 320,
        height: 180,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::default(),
    );
    // You need to set these texture usage flags in order to use the image as a render target
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

    let image_handle = images.add(image);

    // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.
    let first_pass_layer = RenderLayers::layer(1);

    commands.spawn((
        Camera2d::default(),
        Camera {
            target: image_handle.clone().into(),
            clear_color: Color::NONE.into(),
            ..default()
        },
        Transform::default(),
        first_pass_layer.clone(),
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("lyra.png"),
            ..default()
        },
        Transform::default(),
        first_pass_layer,
    ));

    // Main pass cube, with material containing the rendered first pass texture.
    commands.spawn((
        Sprite {
            image: image_handle,
            custom_size: Some(Vec2::new(320.0, 180.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
        ImageMarker,
    ));

    // add capusle 2d
    let capsule_handle = meshes.add(Capsule2d::new(25.0, 50.0));
    let color = materials.add(Color::linear_rgb(1.0, 0.0, 0.0));
    *lighting_assets = LightingAssets {
        capsule_handle: Some(capsule_handle),
        color: Some(color),
    };
}

#[derive(Resource, Default)]
// replace with from_world
struct LightingAssets {
    capsule_handle: Option<Handle<Mesh>>,
    color: Option<Handle<ColorMaterial>>,
}

fn update(
    q_player: Query<&Transform, With<MainCamera>>,
    mut q_image: Query<&mut Transform, (With<ImageMarker>, Without<MainCamera>)>,
) {
    let Ok(camera_pos) = q_player.get_single() else {
        return;
    };
    let Ok(mut image_pos) = q_image.get_single_mut() else {
        return;
    };

    image_pos.translation = camera_pos.translation.with_z(image_pos.translation.z);
}

fn draw_beams(
    mut commands: Commands,
    mut q_segments: Query<&Transform, Added<LightSegmentMarker>>,
    lighting_assets: Res<LightingAssets>,
) {
    for transform in q_segments.iter_mut() {
        commands.spawn((
            Mesh2d(lighting_assets.capsule_handle.clone().unwrap()),
            MeshMaterial2d(lighting_assets.color.clone().unwrap()),
            RenderLayers::layer(1),
            transform.clone(),
        ));
    }
}
