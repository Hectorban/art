[[block]] struct Uniforms {
    u_time: f32;
    u_resolution: vec2<f32>;
};
[[binding(0), group(0)]] var<uniform> uniforms: Uniforms;

[[binding(1), group(0)]] var u_texture: texture_2d<f32>;

[[stage(fragment)]]
fn main([[builtin(fragment_position)]] FragCoord: vec4<f32>) -> [[location(0)]] vec4<f32> {
    let distortionAmount: f32 = 0.1; // Adjust this value to control the amount of distortion
    let distortionSpeed: f32 = 2.0; // Adjust this value to control the speed of the distortion

    let uv: vec2<f32> = FragCoord.xy / uniforms.u_resolution;
    let distortion: f32 = sin(uv.y * 50.0 + uniforms.u_time * distortionSpeed) * distortionAmount;
    let distortedUV: vec2<f32> = vec2<f32>(uv.x + distortion, uv.y);
    let color: vec4<f32> = textureSample(u_texture, default_sampler, distortedUV);
    return color;
}
