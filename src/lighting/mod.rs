use core::f32;

use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    prelude::*,
    render::{
        mesh::PrimitiveTopology,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
        view::RenderLayers,
    },
    sprite::Material2dPlugin,
};
use bevy_rapier2d::prelude::*;
use material::{BlurMaterial, CombineFramesMaterial, FrameMaskMaterial, GradientLightMaterial};

use crate::{
    camera::{move_camera, MainCamera},
    light::segments::LightSegmentMarker,
    player::{match_player::MatchPlayerZ, PlayerMarker},
    shared::GroupLabel,
};

mod material;

pub struct LightingPlugin;
impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<GradientLightMaterial>::default())
            .add_plugins(Material2dPlugin::<CombineFramesMaterial>::default())
            .add_plugins(Material2dPlugin::<FrameMaskMaterial>::default())
            .add_plugins(Material2dPlugin::<BlurMaterial>::default())
            .init_resource::<LightingRenderData>()
            .add_systems(Startup, setup)
            .add_systems(PostUpdate, update_debug_frames_sprite.after(move_camera))
            .add_systems(
                PostUpdate,
                update_light_overlay_position
                    .after(move_camera)
                    .after(draw_beams),
            )
            .add_systems(
                PostUpdate,
                draw_beams.after(move_camera).after(create_occluders),
            )
            .add_systems(PostUpdate, create_occluders.after(move_camera));
    }
}
#[derive(Component)]
pub struct LightRenderLayer;

#[derive(Component)]
pub struct DebugFramesSprite;

#[derive(Resource)]
pub struct LightingRenderData {
    pub gradient_mesh: Handle<Mesh>,
    pub gradient_material: Handle<GradientLightMaterial>,

    pub combine_frames_mesh: Handle<Mesh>,
    pub combine_frames_material: Handle<CombineFramesMaterial>,

    pub blur_mesh: Handle<Mesh>,
    pub blur_material: Handle<BlurMaterial>,

    pub combined_frames_image: Handle<Image>,
    pub frames_image: Handle<Image>,
    pub blurred_image: Handle<Image>,

    pub frame_mask_materials: [Handle<FrameMaskMaterial>; 16],
}

fn new_render_image(width: u32, height: u32) -> Image {
    let mut image = Image::new_fill(
        Extent3d {
            width,
            height,
            ..default()
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;
    image.sampler = ImageSampler::nearest();
    return image;
}

impl FromWorld for LightingRenderData {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();

        let gradient_mesh = meshes
            .add(Mesh::from(Rectangle::new(320. * 4., 180. * 4.)))
            .into();
        let combine_frames_mesh = meshes.add(Mesh::from(Rectangle::new(320., 180.))).into();
        let blur_mesh = meshes.add(Mesh::from(Rectangle::new(320., 180.))).into();

        let mut images = world.resource_mut::<Assets<Image>>();

        let combined_frames_image = images.add(new_render_image(320, 180));
        let blurred_image = images.add(new_render_image(320, 180));
        let frames_image = images.add(new_render_image(320 * 4, 180 * 4));

        let mut materials = world.resource_mut::<Assets<GradientLightMaterial>>();

        let gradient_material = materials.add(GradientLightMaterial {
            light_points: [Vec4::splat(10000000.0); 16], // Position (normalized to screen space)
            light_radius: 40.0,                          // Light radius in pixels
            mesh_transform: Vec4::new(320., 180., 0., 0.),
        });

        let mut materials = world.resource_mut::<Assets<CombineFramesMaterial>>();

        let combine_frames_material = materials.add(CombineFramesMaterial {
            image: frames_image.clone(),
            light_colors: [
                Vec4::new(0., 1.0, 0., 1.0),
                Vec4::new(0., 1.0, 0., 1.0),
                Vec4::new(1.0, 0.3, 0.3, 1.0),
                Vec4::new(1.0, 0.3, 0.3, 1.0),
                Vec4::new(1.0, 0.3, 0.3, 1.0),
                Vec4::new(0.8, 0.8, 0.5, 1.0),
                Vec4::new(0.8, 0.8, 0.5, 1.0),
                Vec4::new(1.0, 1.0, 1.0, 1.0),
                Vec4::new(1.0, 1.0, 1.0, 1.0),
                Vec4::new(1.0, 1.0, 1.0, 1.0),
                Vec4::new(1.0, 1.0, 1.0, 1.0),
                Vec4::new(1.0, 1.0, 1.0, 1.0),
                Vec4::new(1.0, 1.0, 1.0, 1.0),
                Vec4::new(1.0, 1.0, 1.0, 1.0),
                Vec4::new(1.0, 1.0, 1.0, 1.0),
                Vec4::new(1.0, 1.0, 1.0, 1.0),
            ], // Warm light color
        });

        let mut materials = world.resource_mut::<Assets<FrameMaskMaterial>>();
        let frame_mask_materials = (0..16)
            .into_iter()
            .map(|i| {
                materials.add(FrameMaskMaterial {
                    frame_count_x: 4,
                    frame_count_y: 4,
                    frame_index: i,
                    color: Vec4::new(1. - (i as f32 / 16.0), 0.0, i as f32 / 16.0, 1.0),
                })
            })
            .collect::<Vec<_>>();

        let mut materials = world.resource_mut::<Assets<BlurMaterial>>();
        let blur_material = materials.add(BlurMaterial {
            image: combined_frames_image.clone(),
        });

        LightingRenderData {
            gradient_material,
            combine_frames_material,
            gradient_mesh,
            combine_frames_mesh,
            combined_frames_image,
            frames_image,
            frame_mask_materials: frame_mask_materials.try_into().unwrap(),
            blur_mesh,
            blur_material,
            blurred_image,
        }
    }
}

fn setup(mut commands: Commands, lighting_render_data: Res<LightingRenderData>) {
    // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.
    let frames_layer = RenderLayers::layer(1);
    let combined_frames_layer = RenderLayers::layer(2);
    let blurred_layer = RenderLayers::layer(3);

    commands.spawn((
        Camera2d::default(),
        Camera {
            target: lighting_render_data.blurred_image.clone().into(),
            clear_color: Color::NONE.into(),
            ..default()
        },
        Transform::default(),
        blurred_layer.clone(),
    ));

    commands.spawn((
        Camera2d::default(),
        Camera {
            target: lighting_render_data.combined_frames_image.clone().into(),
            clear_color: Color::NONE.into(),
            ..default()
        },
        Transform::default(),
        combined_frames_layer.clone(),
    ));

    commands.spawn((
        Camera2d::default(),
        Camera {
            target: lighting_render_data.frames_image.clone().into(),
            clear_color: Color::NONE.into(),
            ..default()
        },
        Transform::default(),
        frames_layer.clone(),
    ));

    commands.spawn((
        Sprite {
            image: lighting_render_data.blurred_image.clone(),
            custom_size: Some(Vec2::new(320.0, 180.0)),
            ..default()
        },
        Transform::default(),
        LightRenderLayer,
        MatchPlayerZ { offset: 1.5 },
    ));

    // commands.spawn((
    //     Sprite {
    //         image: lighting_render_data.frames_image.clone(),
    //         custom_size: Some(Vec2::new(320.0 / 4.0, 180.0 / 4.0)),
    //         ..default()
    //     },
    //     Transform::default(),
    //     DebugFramesSprite,
    //     MatchPlayerZ { offset: 2.0 },
    // ));

    commands.spawn((
        Mesh2d(lighting_render_data.gradient_mesh.clone()),
        MeshMaterial2d(lighting_render_data.gradient_material.clone()),
        Transform::default(),
        frames_layer.clone(),
    ));

    commands.spawn((
        Mesh2d(lighting_render_data.combine_frames_mesh.clone()),
        MeshMaterial2d(lighting_render_data.combine_frames_material.clone()),
        Transform::default(),
        combined_frames_layer.clone(),
    ));

    commands.spawn((
        Mesh2d(lighting_render_data.blur_mesh.clone()),
        MeshMaterial2d(lighting_render_data.blur_material.clone()),
        Transform::default(),
        blurred_layer.clone(),
    ));
}

fn update_light_overlay_position(
    q_camera: Query<&Transform, With<MainCamera>>,
    mut q_light_layer: Query<&mut Transform, (With<LightRenderLayer>, Without<MainCamera>)>,
) {
    let Ok(camera_pos) = q_camera.get_single() else {
        return;
    };
    let Ok(mut light_layer_pos) = q_light_layer.get_single_mut() else {
        return;
    };

    light_layer_pos.translation = camera_pos.translation.with_z(light_layer_pos.translation.z);
}

fn update_debug_frames_sprite(
    q_camera: Query<&Transform, With<MainCamera>>,
    mut q_light_layer: Query<&mut Transform, (With<DebugFramesSprite>, Without<MainCamera>)>,
) {
    let Ok(camera_pos) = q_camera.get_single() else {
        return;
    };
    let Ok(mut light_layer_pos) = q_light_layer.get_single_mut() else {
        return;
    };

    light_layer_pos.translation = camera_pos.translation.with_z(light_layer_pos.translation.z);
}

fn vector_to_segment(p: Vec2, a: Vec2, b: Vec2) -> Vec2 {
    // Vector from A to P
    let ap = p - a;

    // Vector from A to B
    let ab = b - a;

    // Squared length of AB
    let ab_length_squared = ab.dot(ab);

    // Handle degenerate case: A and B are the same point
    if ab_length_squared == 0.0 {
        return -ap; // The vector directly towards A (or B, since they're the same)
    }

    // Projection of P onto the infinite line defined by A and B
    let t = f32::clamp(ap.dot(ab) / ab_length_squared, 0.0, 1.0); // Clamp t to [0, 1]

    // Closest point on the segment
    let closest_point = a + t * ab;

    // Return vector towards the segment
    return closest_point - p;
}

fn polygon_mesh(vertices: &[Vec2]) -> Mesh {
    let mut triangles = Vec::new();
    for i in 0..(vertices.len() - 1) {
        triangles.extend(
            [
                vertices[0].extend(0.),
                vertices[i].extend(0.),
                vertices[i + 1].extend(0.),
            ]
            .map(|v| [v.x, v.y, v.z]),
        );
    }
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; triangles.len()])
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; triangles.len()])
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, triangles)
}

fn draw_beams(
    q_segments: Query<&Transform, With<LightSegmentMarker>>,
    render: Res<LightingRenderData>,
    mut material: ResMut<Assets<GradientLightMaterial>>,
    q_camera: Query<&Transform, With<MainCamera>>,
    q_occluders: Query<(Entity, &Occluder)>,
    q_player: Query<&Transform, With<PlayerMarker>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let Ok(camera_t) = q_camera.get_single() else {
        return;
    };
    let Some(mat) = material.get_mut(&render.gradient_material) else {
        return;
    };
    let mut light_points = [Vec4::splat(99999999999.); 16];

    let Ok(player_t) = q_player.get_single() else {
        return;
    };
    let player_pos = player_t.translation.truncate();

    let mut lights = q_segments
        .iter()
        .map(|transform| {
            let unit_vec = transform
                .rotation
                .mul_vec3(Vec3::new(1.0, 0.0, 0.0))
                .truncate();
            let pos_1 = transform.translation.truncate() + unit_vec * transform.scale.x / 2.;
            let pos_2 = transform.translation.truncate() - unit_vec * transform.scale.x / 2.;
            (pos_1, pos_2)
        })
        .take(light_points.len())
        .collect::<Vec<_>>();

    lights.push((player_pos, player_pos));
    for (i, (pos_1, pos_2)) in lights.iter().enumerate() {
        light_points[i] = Vec4::new(pos_1.x, pos_1.y, pos_2.x, pos_2.y);
    }
    mat.light_points = light_points;
    let camera_translation = camera_t.translation.truncate();
    mat.mesh_transform.z = camera_translation.x;
    mat.mesh_transform.w = camera_translation.y;

    for (entity, occluder) in q_occluders.iter() {
        let Some((light_pos_1, light_pos_2)) = lights.get(occluder.frame_index) else {
            continue;
        };

        let frame_i = occluder.frame_index % 4;
        let frame_j = occluder.frame_index / 4;

        let point_1 = occluder.point_1;
        let point_2 = occluder.point_2;

        let point_1_to_segment = vector_to_segment(point_1, *light_pos_1, *light_pos_2);
        let point_2_to_segment = vector_to_segment(point_2, *light_pos_1, *light_pos_2);

        let camera_point = |p: Vec2| p - camera_translation + Vec2::new(320.0 * 0.5, -180. * 0.5);

        let out_of_bounds = |p: Vec2| {
            let x = p.x;
            let y = p.y;
            let buffer = 4.0;
            !(x > -buffer && x < 320. + buffer && y < buffer && y > -180.0 - buffer)
        };

        let frame_point = |p: Vec2| {
            camera_point(p)
                + Vec2::new(320.0 * -2., 180. * 2.)
                + Vec2::new(320. * frame_i as f32, -180. * frame_j as f32)
        };

        let point_1_frame = frame_point(point_1);

        let pos = point_1_frame;

        let is_out_of_bounds =
            out_of_bounds(camera_point(point_1)) || out_of_bounds(camera_point(point_2));

        let is_out_of_lights = point_1_to_segment.length() > 40.0
            && point_1_to_segment.length() > 40.0
            && (point_1 - point_2).length() < 40.0;

        if is_out_of_bounds || is_out_of_lights {
            commands.entity(entity).remove::<Mesh2d>();
            continue;
        }

        let Some(occluder_polygon) =
            create_occluder_polygon(point_1, point_2, point_1_to_segment, point_2_to_segment)
        else {
            commands.entity(entity).remove::<Mesh2d>();
            continue;
        };

        let shape = meshes.add(polygon_mesh(
            &occluder_polygon
                .into_iter()
                .map(|x| x - point_1)
                .collect::<Vec<_>>(),
        ));
        commands.entity(entity).insert((
            Transform::default().with_translation(pos.extend(1.0)),
            Mesh2d(shape),
        ));
    }
}

fn create_occluder_polygon(
    point_1: Vec2,
    point_2: Vec2,
    point_1_to_segment: Vec2,
    point_2_to_segment: Vec2,
) -> Option<Vec<Vec2>> {
    let polygon = [
        point_1,
        point_2,
        point_2 - point_2_to_segment.normalize() * 300.0,
        point_1 - point_1_to_segment.normalize() * 300.0,
    ];

    Some(polygon.into())
}

#[derive(Component)]
pub struct Occluder {
    pub point_1: Vec2,
    pub point_2: Vec2,
    pub frame_index: usize,
}

fn create_occluders(
    mut commands: Commands,
    q_occluders: Query<(&GlobalTransform, &Collider, &CollisionGroups), Added<Collider>>,
    render: Res<LightingRenderData>,
) {
    for (translation, (point_1, point_2)) in q_occluders
        .iter()
        .filter_map(|(transform, collider, groups)| {
            if groups.memberships & GroupLabel::TERRAIN == Group::NONE {
                return None;
            };
            match collider.as_typed_shape() {
                ColliderView::Cuboid(cub) => {
                    let (half_x, half_y) = cub.half_extents().into();
                    let four_corners = [(-1., -1.), (-1., 1.), (1., 1.), (1., -1.)]
                        .map(|(x, y)| Vec2::new(half_x * x, half_y * y));
                    return Some((
                        transform.translation(),
                        [
                            (four_corners[0], four_corners[1]),
                            (four_corners[1], four_corners[2]),
                            (four_corners[2], four_corners[3]),
                            (four_corners[3], four_corners[0]),
                        ],
                    ));
                }
                _ => None,
            }
        })
        .flat_map(|(translation, sides)| sides.into_iter().map(move |side| (translation, side)))
    {
        for frame_index in 0..16 {
            commands.spawn((
                Occluder {
                    point_1: translation.truncate() + point_1,
                    point_2: translation.truncate() + point_2,
                    frame_index,
                },
                RenderLayers::layer(1),
                MeshMaterial2d(render.frame_mask_materials[frame_index].clone()),
            ));
        }
    }
}
