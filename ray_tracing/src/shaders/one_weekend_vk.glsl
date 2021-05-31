#version 450

#include "common/base.glsl"

// layout

layout(local_size_x = 32, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) uniform Config {
    float image_width;
    float image_height;
} opData;

layout(set = 0, binding = 1) buffer ColorBuffer {
    vec4 data[];
} colorData;

layout(set = 0, binding = 2) buffer PositionBuffer {
    attribute_data data[];
} positionData;

layout(constant_id = 0) const uint primitive_count = 0;

// functions

#include "common/hit_triangle.glsl"

vec3 ray_color(in ray r) {
    for (int i = 0; i < primitive_count; i++) {
        attribute_data position = positionData.data[i];
        attribute_data normal = attribute_data(vec3(0), vec3(0), vec3(0));
        attribute_data uv = attribute_data(vec3(0), vec3(0), vec3(0));

        triangle tr = triangle(position, normal, uv);

        hit_record rec;

        rec.r = r;
        rec.t_min = 0.0;
        rec.t_max = 1000.0;

        if (hit_triangle(tr, rec)) {
            return vec3(1.0, 0.0, 0.0);
        }
    }

    vec3 unit_direction = normalize(r.dir);
    float t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0);
}

// main

void main() {
    uint idx = gl_GlobalInvocationID.x;
    
    #include "common/calculate_uv.glsl"
    #include "common/calculate_ray.glsl"

    vec3 pixel_color = ray_color(r);

    colorData.data[idx] = vec4(pixel_color, 1.0);
}