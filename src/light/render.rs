use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d},
};
use enum_map::{enum_map, EnumMap};

use super::{segments::LightSegment, LightColor, LIGHT_SEGMENT_THICKNESS};

const LIGHT_SHADER_PATH: &str = "shaders/light.wgsl";

pub struct LightRenderData {
    pub mesh: Mesh2d,
    pub material_map: EnumMap<LightColor, MeshMaterial2d<LightMaterial>>,
}

impl FromWorld for LightRenderData {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let mesh_handle = meshes
            .add(Rectangle::new(1.0, LIGHT_SEGMENT_THICKNESS))
            .into();

        let mut materials = world.resource_mut::<Assets<LightMaterial>>();

        LightRenderData {
            mesh: mesh_handle,
            material_map: enum_map! {
                LightColor::Green => materials.add(LightMaterial::from(LightColor::Green)).into(),
                LightColor::Red => materials.add(LightMaterial::from(LightColor::Red)).into(),
                LightColor::White => materials.add(LightMaterial::from(LightColor::White)).into(),
            },
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct LightMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    pub alpha_mode: AlphaMode2d,
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
    visibility: Visibility,
}

pub fn insert_segment_meshes(
    mut commands: Commands,
    render_data: Local<LightRenderData>,
    q_segments: Query<(Entity, &LightSegment), Added<LightSegment>>,
) {
    let segs: Vec<(Entity, LightSegmentRenderBundle)> = q_segments
        .iter()
        .map(|(entity, segment)| {
            let segment_bundle = LightSegmentRenderBundle {
                mesh: render_data.mesh.clone(),
                material: render_data.material_map[segment.color].clone(),
                visibility: Visibility::Visible,
            };
            (entity, segment_bundle)
        })
        .collect();

    commands.insert_batch(segs);
}
