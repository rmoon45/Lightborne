#import bevy_render::globals::Globals
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var light_image: texture_2d<f32>;
@group(2) @binding(1) var light_sampler: sampler;


@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let mesh_uv: vec2<f32> = mesh.uv;

    let light_color = textureSample(light_image, light_sampler, mesh_uv);

    return light_color;
}
