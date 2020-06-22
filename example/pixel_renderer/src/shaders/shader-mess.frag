#version 450
// no idea what this does
#extension GL_ARB_separate_shader_objects : enable

// output to framebuffer at index 0
layout(location = 0) out vec4 out_color;

layout(location = 1) in vec2 frag_tex_coord;

layout(binding = 0) uniform sampler2D tex_sampler;

// const float weight[5] = float[] (0.227027, 0.1945946, 0.1216216, 0.054054, 0.016216);
// const float weights[5][5] = float[][] (weight, weight, weight, weight, weight);

const float weights[9][9] = float[][] (float[] (0, 0.000001, 0.000014, 0.000055, 0.000088, 0.000055, 0.000014, 0.000001, 0),
                                       float[] (0.000001, 0.000036, 0.000362, 0.001445, 0.002289, 0.001445, 0.000362, 0.000036, 0.000001),
                                       float[] (0.000014, 0.000362, 0.003672, 0.014648, 0.023205, 0.014648, 0.003672, 0.000362, 0.000014),
                                       float[] (0.000055, 0.001445, 0.014648, 0.058434, 0.092566, 0.058434, 0.014648, 0.001445, 0.000055),
                                       float[] (0.000088, 0.002289, 0.023205, 0.092566, 0.146634, 0.092566, 0.023205, 0.002289, 0.000088),
                                       float[] (0.000055, 0.001445, 0.014648, 0.058434, 0.092566, 0.058434, 0.014648, 0.001445, 0.000055),
                                       float[] (0.000014, 0.000362, 0.003672, 0.014648, 0.023205, 0.014648, 0.003672, 0.000362, 0.000014),
                                       float[] (0.000001, 0.000036, 0.000362, 0.001445, 0.002289, 0.001445, 0.000362, 0.000036, 0.000001),
                                       float[] (0, 0.000001, 0.000014, 0.000055, 0.000088, 0.000055, 0.000014, 0.000001, 0));



void main()
{
    // vec2 tex_offset = 1.0 / (textureSize(tex_sampler, 0) * 8.0);
    vec2 tex_offset = 1.0 / vec2(1768, 992);
    float result = 0.0; //  = texture(tex_sampler, frag_tex_coord).r * weight[0];

    for(int y = 0; y < 9; ++y) {
        for(int x = 0; x < 9; ++x) {
            result += texture(tex_sampler, frag_tex_coord - tex_offset * 4
                              + vec2(tex_offset.x * x, tex_offset.y * y)).r * weights[y][x];
            // result += texture(tex_sampler, frag_tex_coord - tex_offset * 4
            //                   - vec2(tex_offset.x * x, tex_offset.y * y)).r * weights[y][x];
        }
    }

    // for(int i = 0; i < 1; ++i) {
    //     for(int i = 1; i < 5; ++i) {
    //         // horizontal
    //         result += texture(tex_sampler, frag_tex_coord + vec2(tex_offset.x * i, 0.0)).r * weight[i];
    //         result += texture(tex_sampler, frag_tex_coord - vec2(tex_offset.x * i, 0.0)).r * weight[i];

    //         //  vertical
    //         result += texture(tex_sampler, frag_tex_coord + vec2(0.0, tex_offset.y * i)).r * weight[i];
    //         result += texture(tex_sampler, frag_tex_coord - vec2(0.0, tex_offset.y * i)).r * weight[i];
    //     }
    // }

    out_color = vec4(result.rrr, 1.0);
}

// void main()
// {
//     vec2 tex_offset = 1.0 / (textureSize(tex_sampler, 0) * 25.0);
//     float result = texture(tex_sampler, frag_tex_coord).r * weight[0];
// //
//     for(int i = 0; i < 1; ++i) {
//         for(int i = 1; i < 5; ++i) {
//             // horizontal
//             result += texture(tex_sampler, frag_tex_coord + vec2(tex_offset.x * i, 0.0)).r * weight[i];
//             result += texture(tex_sampler, frag_tex_coord - vec2(tex_offset.x * i, 0.0)).r * weight[i];
// //
//             // vertical
//             result += texture(tex_sampler, frag_tex_coord + vec2(0.0, tex_offset.y * i)).r * weight[i];
//             result += texture(tex_sampler, frag_tex_coord - vec2(0.0, tex_offset.y * i)).r * weight[i];
//         }
//     }
// //
//     out_color = vec4(result.rrr, 1.0);
// }

// void main()
// {
//     // vec2 ps = vec2(1/32, 1/16);
//     vec2 ps = 1.0 / (textureSize(tex_sampler, 0) * 10);
// //
//     //  cond assignm ????????!!01011
//     // float glow_threshold = 0.0;
//     float col0 = max(texture(tex_sampler, frag_tex_coord + vec2(-ps.x, 0.0)).r, 0.0); // 1
//     float col1 = max(texture(tex_sampler, frag_tex_coord + vec2(ps.x, 0.0)).r, 0.0);
//     float col2 = max(texture(tex_sampler, frag_tex_coord + vec2(0.0, -ps.y)).r, 0.0);
//     float col3 = max(texture(tex_sampler, frag_tex_coord + vec2(0.0, ps.y)).r, 0.0);
//     //
//     // out_color = vec4(0.5 * (col0 + col1 + col2 + col3).rrr, 1.0);
//     //
//     vec4 color = texture(tex_sampler, frag_tex_coord);
//     float glowing_color = 0.25 * (col0 + col1 + col2 + col3);
// //
//     out_color = vec4(glowing_color.rrr, 1.0);
// //
//     //  out_color = vec4(texture(tex_sampler, frag_tex_coord).rrr, 1.0);
// }

// void main()
// {
//     vec2 tex_offset = 1.0 / (textureSize(tex_sampler, 0) * 15.0);
//     float result = texture(tex_sampler, frag_tex_coord).r;
//
//     result += texture(tex_sampler, vec2(frag_tex_coord.x - 14.0 * tex_offset.x, frag_tex_coord.y)).r * 0.000000115484001;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x - 13.0 * tex_offset.x, frag_tex_coord.y)).r * 0.00000115484001;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x - 12.0 * tex_offset.x, frag_tex_coord.y)).r * 0.000008372590071;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x - 11.0 * tex_offset.x, frag_tex_coord.y)).r * 0.0000468865044;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x - 10.0 * tex_offset.x, frag_tex_coord.y)).r * 0.0002109892698;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x - 9.0 * tex_offset.x, frag_tex_coord.y)).r * 0.0007836744306;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x - 8.0 * tex_offset.x, frag_tex_coord.y)).r * 0.002448982596;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x - 7.0 * tex_offset.x, frag_tex_coord.y)).r * 0.006530620255;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x - 6.0 * tex_offset.x, frag_tex_coord.y)).r * 0.01502042659;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x - 5.0 * tex_offset.x, frag_tex_coord.y)).r * 0.03004085317;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x - 4.0 * tex_offset.x, frag_tex_coord.y)).r * 0.05257149305;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x - 3.0 * tex_offset.x, frag_tex_coord.y)).r * 0.08087922008;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x - 2.0 * tex_offset.x, frag_tex_coord.y)).r * 0.1097646558;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x - 1.0 * tex_offset.x, frag_tex_coord.y)).r * 0.131717587;
//
//     result += texture(tex_sampler, frag_tex_coord).r * 0.1399499362;
//
//     result += texture(tex_sampler, vec2(frag_tex_coord.x + 1.0 * tex_offset.x, frag_tex_coord.y)).r * 0.131717587;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x + 2.0 * tex_offset.x, frag_tex_coord.y)).r * 0.1097646558;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x + 3.0 * tex_offset.x, frag_tex_coord.y)).r * 0.08087922008;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x + 4.0 * tex_offset.x, frag_tex_coord.y)).r * 0.05257149305;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x + 5.0 * tex_offset.x, frag_tex_coord.y)).r * 0.03004085317;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x + 6.0 * tex_offset.x, frag_tex_coord.y)).r * 0.01502042659;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x + 7.0 * tex_offset.x, frag_tex_coord.y)).r * 0.006530620255;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x + 8.0 * tex_offset.x, frag_tex_coord.y)).r * 0.002448982596;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x + 9.0 * tex_offset.x, frag_tex_coord.y)).r * 0.0007836744306;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x + 10.0 * tex_offset.x, frag_tex_coord.y)).r * 0.0002109892698;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x + 11.0 * tex_offset.x, frag_tex_coord.y)).r * 0.0000468865044;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x + 12.0 * tex_offset.x, frag_tex_coord.y)).r * 0.000008372590071;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x + 13.0 * tex_offset.x, frag_tex_coord.y)).r * 0.00000115484001;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x + 14.0 * tex_offset.x, frag_tex_coord.y)).r * 0.000000115484001;
//
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y - 14.0 * tex_offset.y)).r * 0.000000115484001;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y - 13.0 * tex_offset.y)).r * 0.00000115484001;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y - 12.0 * tex_offset.y)).r * 0.000008372590071;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y - 11.0 * tex_offset.y)).r * 0.0000468865044;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y - 10.0 * tex_offset.y)).r * 0.0002109892698;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y - 9.0 * tex_offset.y)).r * 0.0007836744306;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y - 8.0 * tex_offset.y)).r * 0.002448982596;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y - 7.0 * tex_offset.y)).r * 0.006530620255;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y - 6.0 * tex_offset.y)).r * 0.01502042659;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y - 5.0 * tex_offset.y)).r * 0.03004085317;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y - 4.0 * tex_offset.y)).r * 0.05257149305;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y - 3.0 * tex_offset.y)).r * 0.08087922008;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y - 2.0 * tex_offset.y)).r * 0.1097646558;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y - 1.0 * tex_offset.y)).r * 0.131717587;
//
//     result += texture(tex_sampler, frag_tex_coord).r * 0.1399499362;
//
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y + 1.0 * tex_offset.y)).r * 0.131717587;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y + 2.0 * tex_offset.y)).r * 0.1097646558;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y + 3.0 * tex_offset.y)).r * 0.08087922008;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y + 4.0 * tex_offset.y)).r * 0.05257149305;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y + 5.0 * tex_offset.y)).r * 0.03004085317;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y + 6.0 * tex_offset.y)).r * 0.01502042659;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y + 7.0 * tex_offset.y)).r * 0.006530620255;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y + 8.0 * tex_offset.y)).r * 0.002448982596;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y + 9.0 * tex_offset.y)).r * 0.0007836744306;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y + 10.0 * tex_offset.y)).r * 0.0002109892698;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y + 11.0 * tex_offset.y)).r * 0.0000468865044;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y + 12.0 * tex_offset.y)).r * 0.000008372590071;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y + 13.0 * tex_offset.y)).r * 0.00000115484001;
//     result += texture(tex_sampler, vec2(frag_tex_coord.x, frag_tex_coord.y + 14.0 * tex_offset.y)).r * 0.000000115484001;
//
//     out_color = vec4(result.rrr, 1.0);
//
// }



// vec2 ps = vec2(1/128, 1/64);
// vec4 col = vec4(texture(tex_sampler, frag_tex_coord).rrr, 1.0);
// vec4 glow = col + 1;
// float r = 5.0;
// float amount = 0.25;

// glow += texture(tex_sampler, frag_tex_coord + vec2(-r, -r) * ps);
// glow += texture(tex_sampler, frag_tex_coord + vec2(-r, 0.0) * ps);
// glow += texture(tex_sampler, frag_tex_coord + vec2(-r, r) * ps);
// glow += texture(tex_sampler, frag_tex_coord + vec2(0.0, -r) * ps);
// glow += texture(tex_sampler, frag_tex_coord + vec2(0.0, r) * ps);
// glow += texture(tex_sampler, frag_tex_coord + vec2(r, -r) * ps);
// glow += texture(tex_sampler, frag_tex_coord + vec2(r, 0.0) * ps);
// glow += texture(tex_sampler, frag_tex_coord + vec2(r, r) * ps);

// r *= 2.0;
// glow += texture(tex_sampler, frag_tex_coord + vec2(-r, -r) * ps);
// glow += texture(tex_sampler, frag_tex_coord + vec2(-r, 0.0) * ps);
// glow += texture(tex_sampler, frag_tex_coord + vec2(-r, r) * ps);
// glow += texture(tex_sampler, frag_tex_coord + vec2(0.0, -r) * ps);
// glow += texture(tex_sampler, frag_tex_coord + vec2(0.0, r) * ps);
// glow += texture(tex_sampler, frag_tex_coord + vec2(r, -r) * ps);
// glow += texture(tex_sampler, frag_tex_coord + vec2(r, 0.0) * ps);
// glow += texture(tex_sampler, frag_tex_coord + vec2(r, r) * ps);

// // glow /= 17.0;
// glow *= amount;
// col.rgb *= col.a;

// out_color = glow + col;
