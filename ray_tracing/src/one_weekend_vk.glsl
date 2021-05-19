#version 450

// layout

layout(local_size_x = 32, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) uniform Config {
    float image_width;
    float image_height;
} opData;

layout(set = 0, binding = 1) buffer ColorBuffer {
    vec4 data[];
} colorData;

struct Primitive {
    vec4 a;
    vec4 b;
    vec4 c;
    vec4 d;
};

layout(set = 0, binding = 2) buffer PrimitivesBuffer {
    Primitive data[];
} primitivesData;

layout(constant_id = 0) const uint primitive_count = 0;

// functions

vec2 getUV(uint i) {
    uint x = i % uint(opData.image_width);
    uint y = (i - x) / uint(opData.image_width);

    float u = float(x) / (opData.image_width - 1);
    float vv = float(y) / (opData.image_height - 1);
    float v = 1.0 - vv;

    return vec2(u, v);
}

bool isSphere(Primitive p) {
    return p.d.x == 0.0;
}

// main

void main() {
    uint idx = gl_GlobalInvocationID.x;
    vec2 uv = getUV(idx);
    Primitive p = primitivesData.data[0];

    for (uint i = 0; i < primitive_count; i++) {
        colorData.data[idx] = vec4(isSphere(p) ? 1.0 : 0.0, 0.0, 0.0, 1.0);
    }
}