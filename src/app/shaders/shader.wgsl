// Vertex shader




struct CameraUniform 
{
    view_proj: mat4x4<f32>,
};


@group(0) @binding(0) // 1.
var<uniform> camera: CameraUniform;
// Vertex shader





struct VertexInput 
{
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) normal: vec3<f32>,
}

struct InstanceInput 
{
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
};


struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) normal: vec3<f32>,
}




@vertex
fn vs_main( model: VertexInput, instance: InstanceInput) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );    


    var out: VertexOutput;
    out.uv = model.uv;
    out.clip_position =  camera.view_proj * model_matrix *  vec4<f32>(model.position, 1.0);
    out.normal = model.normal;
    return out;
}


struct Color
{
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> c_diffuse: Color;
@group(1) @binding(1)
var t_diffuse: texture_2d<f32>;
@group(1)@binding(2)
var s_diffuse: sampler;





@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.uv);
    //return vec4<f32>(c_diffuse.color);
}

/*
struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
}
@group(2) @binding(0)
var<uniform> light: Light;



@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let object_color: vec4<f32> = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    
    // We don't need (or want) much ambient light, so 0.1 is fine
    let ambient_strength = 0.1;
    let ambient_color = light.color * ambient_strength;

    let result = ambient_color * object_color.xyz;

    return vec4<f32>(result, object_color.a);
}

*/


