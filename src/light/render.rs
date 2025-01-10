use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d},
};

use super::{LightSegment, SEGMENT_THICKNESS};

const LIGHT_SHADER_PATH: &str = "shaders/light.wgsl";

#[derive(Resource)]
pub struct LightRenderData {
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<LightMaterial>,
}

impl FromWorld for LightRenderData {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let mesh_handle = Mesh2d(meshes.add(Rectangle::new(1.0, SEGMENT_THICKNESS)));

        let mut materials = world.resource_mut::<Assets<LightMaterial>>();
        let material_handle = MeshMaterial2d(materials.add(LightMaterial {
            color: Color::srgb(5.0, 0.0, 3.0).into(),
            alpha_mode: AlphaMode2d::Blend,
        }));

        LightRenderData {
            mesh: mesh_handle,
            material: material_handle,
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct LightMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    alpha_mode: AlphaMode2d,
}

impl Material2d for LightMaterial {
    fn fragment_shader() -> ShaderRef {
        LIGHT_SHADER_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        self.alpha_mode
    }
}

#[derive(Bundle, Default, Debug)]
pub struct LightSegmentRenderBundle {
    mesh: Mesh2d,
    material: MeshMaterial2d<LightMaterial>,
    transform: Transform,
    visibility: Visibility,
}

pub fn insert_segment_meshes(
    mut commands: Commands,
    render_data: Res<LightRenderData>,
    q_segments: Query<(Entity, &LightSegment)>,
) {
    let segs = q_segments
        .iter()
        .map(|(entity, segment)| {
            let midpoint = segment.start.midpoint(segment.end).extend(1.0);
            let scale = Vec3::new(segment.start.distance(segment.end), 1., 1.);
            let rotation = (segment.end - segment.start).to_angle();

            let segment_bundle = LightSegmentRenderBundle {
                mesh: render_data.mesh.clone(),
                material: render_data.material.clone(),
                transform: Transform::from_translation(midpoint)
                    .with_scale(scale)
                    .with_rotation(Quat::from_rotation_z(rotation)),
                visibility: Visibility::Visible,
            };
            (entity, segment_bundle)
        })
        .collect::<Vec<(Entity, LightSegmentRenderBundle)>>();

    commands.insert_batch(segs);
}

pub fn clean_segments(mut commands: Commands, q_segment_meshes: Query<Entity, With<LightSegment>>) {
    for entity in q_segment_meshes.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
