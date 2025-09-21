/// Shader taken from https://github.com/patriciogonzalezvivo/lygia_p5_examples/blob/main/data/generative_pnoise.frag
/// Adapted to run in the playground: https://play.wesl-lang.dev/

import lygia::generative::pnoise::{pnoise2, pnoise3};

struct Uniforms {
  /// canvas dimensions, in pixels
  size: vec2f,
  /// mouse position on the canvas, in pixels
  mouse: vec2f,
  /// timestamp, in seconds
  time: f32,
  /// delta since previous frame, in seconds
  delta: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@fragment
fn main(@builtin(position) pos: vec4f) -> @location(0) vec4f {
    var color = vec4(vec3(0.0), 1.0);
    let pixel = 1.0/uniforms.size;
    let st = pos.xy * pixel;

    let d2 = pnoise2(vec2(st * 5. + uniforms.time), vec2(1.2, 3.4)) * 0.5 + 0.5;
    let d3 = pnoise3(vec3(st * 5., uniforms.time), vec3(1.2, 3.4, 5.6)) * 0.5 + 0.5;
    
    color += mix(d2, d3, step(0.5, st.x));

    return color;
}
