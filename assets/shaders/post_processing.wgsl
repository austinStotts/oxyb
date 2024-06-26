// This shader computes the chromatic aberration effect

// Since post processing is a fullscreen effect, we use the fullscreen vertex shader provided by bevy.
// This will import a vertex shader that renders a single fullscreen triangle.
//
// A fullscreen triangle is a single triangle that covers the entire screen.
// The box in the top left in that diagram is the screen. The 4 x are the corner of the screen
//
// Y axis
//  1 |  x-----x......
//  0 |  |  s  |  . ´
// -1 |  x_____x´
// -2 |  :  .´
// -3 |  :´
//    +---------------  X axis
//      -1  0  1  2  3
//
// As you can see, the triangle ends up bigger than the screen.
//
// You don't need to worry about this too much since bevy will compute the correct UVs for you.
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    intensity: f32,
    sigma1: f32,
    tau: f32,
    gfact: f32,
    epsilon: f32,
    num_gvf_iterations: i32,
    enable_xdog: u32,
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Chromatic aberration strength
    let offset_strength = settings.intensity;

    // Sample each color channel with an arbitrary shift
    return vec4<f32>(
        textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(offset_strength, -offset_strength)).r,
        textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(-offset_strength, 0.0)).g,
        textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(0.0, offset_strength)).b,
        1.0
    );
}





// @fragment
// fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {


//     var color = textureSample(screen_texture, texture_sampler, in.uv);
//     var original_color = color;

//     var sigma2 = settings.sigma1 / 16.0;
//     var radius1 = settings.sigma1 * 3.0;
//     var radius2 = sigma2 * 2.0;

//     if (settings.enable_xdog == 1u) {
//         // Gradient calculation using Sobel operators
//          var sobel_x = array<vec3<f32>, 3>(
//              vec3<f32>(-1.0, 0.0, 1.0),
//              vec3<f32>(-2.0, 0.0, 2.0),
//              vec3<f32>(-1.0, 0.0, 1.0)
//          ); 

//          var sobel_y = array<vec3<f32>, 3>(
//              vec3<f32>(-1.0, -2.0, -1.0),
//              vec3<f32>( 0.0,  0.0,  0.0),
//              vec3<f32>( 1.0,  2.0,  1.0)
//          );

//         var dx = 0.0;
//         var dy = 0.0;

//         for (var i: i32 = -1; i <= 1; i++) {
//             for (var j: i32 = -1; j <= 1; j++) {
//                 var offset : vec2<f32> = vec2<f32>(f32(i), f32(j)) / vec2<f32>(textureDimensions(screen_texture));          
//                 var sampleColor = textureSample(screen_texture, texture_sampler, in.uv + offset);
//                 dx += sampleColor.r * sobel_x[i + 1][j + 1]; 
//                 dy += sampleColor.r * sobel_y[i + 1][j + 1];
//             }
//         }

//         // Variables to store GVF (initialized to 0.0)
//         var u: f32 = 0.0;
//         var v: f32 = 0.0;

//         // Perform GVF iterations (replace 'num_gvf_iterations' with your desired count)
//         for (var i = 0; i < settings.num_gvf_iterations; i++) {
//             // Update equations (using dx and dy for gradients)
//             let u_temp = dx + settings.gfact * u;
//             let v_temp = dy + settings.gfact * v;
            
//             // Normalize (sqrt function might require additional implementation)
//             // let norm_factor = approximate_inversesqrt(u_temp * u_temp + v_temp * v_temp + settings.epsilon * settings.epsilon);
//             let norm_factor = sqrt(u_temp * u_temp + v_temp * v_temp + settings.epsilon);
//             u = u_temp / norm_factor;
//             v = v_temp / norm_factor;
//         }

//         // Gaussian Blur Implementation
//         var kernelSize1 : i32 = i32(ceil(radius1) * 2.0 + 1.0); 
//         var kernelSize2 : i32 = i32(ceil(radius2) * 2.0 + 1.0);
//         var blurredImage1 = color; 
//         var blurredImage2 = color; 
//         var total_weight: f32 = 0.0;

//         // Blur 1 Horizontal Pass
//         for (var offsetX : i32 = -kernelSize1 / 2; offsetX <= kernelSize1 / 2; offsetX++) {
//             let samplePos: vec2<f32> = in.uv + vec2<f32>(f32(offsetX) / f32(textureDimensions(screen_texture).x), 0.0);
//             let weight: f32 = exp(-(f32(offsetX) * f32(offsetX)) / (2.0 * settings.sigma1 * settings.sigma1)) / (sqrt(2.0 * 3.14159) * settings.sigma1);
//             blurredImage1 += textureSample(screen_texture, texture_sampler, samplePos) * weight;
//             total_weight += weight; // Keep track of total weight
//         }


//         // Blur 1 Vertical Pass
//         for (var offsetY : i32 = -kernelSize1 / 2; offsetY <= kernelSize1 / 2; offsetY++) {
//             let samplePos: vec2<f32> = in.uv + vec2<f32>(0.0, f32(offsetY) / f32(textureDimensions(screen_texture).y));
//             let weight: f32 = exp(-(f32(offsetY) * f32(offsetY)) / (2.0 * settings.sigma1 * settings.sigma1)) / (sqrt(2.0 * 3.14159) * settings.sigma1);
//             blurredImage1 += textureSample(screen_texture, texture_sampler, samplePos) * weight;
//             total_weight += weight; // Keep track of total weight
//         }


//         // Blur 2 Horizontal Pass
//         for (var offsetX : i32 = -kernelSize2 / 2; offsetX <= kernelSize2 / 2; offsetX++) {
//             let samplePos: vec2<f32> = in.uv + vec2<f32>(f32(offsetX) / f32(textureDimensions(screen_texture).x), 0.0);
//             let weight: f32 = exp(-(f32(offsetX) * f32(offsetX)) / (2.0 * sigma2 * sigma2)) / (sqrt(2.0 * 3.14159) * sigma2);
//             blurredImage2 += textureSample(screen_texture, texture_sampler, samplePos) * weight;
//             total_weight += weight; // Keep track of total weight
//         }


//         // Blur 2 Vertical Pass
//         for (var offsetY : i32 = -kernelSize2 / 2; offsetY <= kernelSize2 / 2; offsetY++) {
//             let samplePos: vec2<f32> = in.uv + vec2<f32>(0.0, f32(offsetY) / f32(textureDimensions(screen_texture).y));
//             let weight: f32 = exp(-(f32(offsetY) * f32(offsetY)) / (2.0 * sigma2 * sigma2)) / (sqrt(2.0 * 3.14159) * sigma2);
//             blurredImage2 += textureSample(screen_texture, texture_sampler, samplePos) * weight;
//             total_weight += weight; // Keep track of total weight
//         }

//         blurredImage1 /= total_weight;
//         blurredImage2 /= total_weight;

//         var xdog_difference = blurredImage2.r - blurredImage1.r; // Assumes only using red channel


//         // Optional Thresholding  
//         if (abs(xdog_difference) >= settings.tau) {
//              xdog_difference = 1.0 - exp(-xdog_difference / settings.tau);
//         } else {
//             xdog_difference = 0.0;
//         }

        

//         color.r = xdog_difference;
//         color.g = color.r;
//         color.b = color.r;

//     }

//     return vec4<f32>(color.r, color.g, color.b, 1.0);
// }