struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
}

struct Material {
    diffuse_color: vec4<f32>,
    diffuse_texture: texture_2d<f32>,
    diffuse_texture_sampler: sampler,
    normal_texture: texture_2d<f32>,
    normal_texture_sampler: sampler,
    reflectance: f32,
    metalness: f32
}

struct FragmentData {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) tangent_position: vec3<f32>,
    @location(2) tangent_light_position: vec3<f32>,
    @location(3) tangent_view_position: vec3<f32>,
    @binding(0) material: Material
};

@group(2) @binding(0)
var<uniform> light: Light;

@group(3) @binding(0)
var material: Material;


@fragment
fn fs_main(in: FragmentData) -> vec4<f32> {
    let normal: vec4<f32> = (textureSample(material.normal_texture_sampler, material.normal_texture, in.tex_coords).rgb() * 2.0 - 1.0).normalize();
    let diffuse: vec4<f32> = textureSample(material.diffuse_texture_sampler, material.diffuse_texture, in.tex_coords).rgb();
    
    let tangent_normal = object_normal.xyz * 2.0 - 1.0;
    let light_dir = normalize(in.tangent_light_position - in.tangent_position);
    let view_dir = normalize(in.tangent_view_position - in.tangent_position);

    let half_dir = normalize(view_dir + light_dir);

    let diffuse_strength = max(dot(tangent_normal, light_dir), 0.0);
    let diffuse_color = light.color * diffuse_strength;

    let specular_strength = pow(max(dot(tangent_normal, half_dir), 0.0), 32.0);
    let specular_color = specular_strength * light.color * material.metalness;

    let result = (ambient_color + diffuse_color + specular_color) * object_color.xyz;

    return vec4<f32>(result, object_color.a);
}