#version 450

// base 

struct Camera {
    vec3 origin; // 0
    vec3 lower_left_corner; //4
    vec3 horizontal; // 8
    vec3 vertical; // 12
    vec3 u; // 16
    vec3 v; // 20
    float lens_radius;
};

struct AttributeData {
    vec3 a;
    vec3 b;
    vec3 c;
};

struct triangle {
   AttributeData position;
   AttributeData normal;
   AttributeData uv;
};

struct Ray {
    vec3 origin;
    vec3 dir;
};

struct HitRecord {
    Ray r;
    float t_min;
    float t_max;
    vec3 p;
    vec3 normal;
    float t;
    float u;
    float v;
    bool front_face;
};

vec3 at(Ray r, float t) {
    return r.origin + r.dir * t;
}

void set_front_face_and_normal(in out HitRecord rec, bool front_face, vec3 outward_normal) {
    rec.front_face = front_face;
    rec.normal = rec.front_face ? outward_normal : -outward_normal;
}

// layout

layout(local_size_x = 32, local_size_y = 1, local_size_z = 1) in;

layout(std140, set = 0, binding = 0) uniform Config {
    float image_width;       // 0
    float image_height;      // 1
    float samples_per_pixel; // 2
    float max_depth;         // 3
    Camera camera;           // 4
};

layout(set = 0, binding = 1) buffer ColorBuffer {
    vec4 colorBuffer[];
};

layout(set = 0, binding = 2) buffer PositionBuffer {
    AttributeData positionData[];
};

layout(constant_id = 0) const uint primitive_count = 0;

// functions

float rand(vec2 co){
    return fract(sin(dot(co, vec2(12.9898, 78.233))) * 43758.5453);
}

bool hit_triangle(in triangle tr, in out HitRecord rec) {
    vec3 a = tr.position.a;
    vec3 b = tr.position.b;
    vec3 c = tr.position.c;

    vec3 na = tr.normal.a;
    vec3 nb = tr.normal.b;
    vec3 nc = tr.normal.c;

    vec2 ta = tr.uv.a.xy;
    vec2 tb = tr.uv.b.xy;
    vec2 tc = tr.uv.c.xy;

    Ray r = rec.r;

    vec3 e1 = b - a;
    vec3 e2 = c - a;
    vec3 x = cross(r.dir, e2);
    float d = dot(e1, x);
    float eps = 1e-6;

    if (d > -eps && d < eps) {
        return false;
    }

    float f = 1.0 / d;
    vec3 s = r.origin - a;
    vec3 y = cross(s, e1);
    float t = f * dot(e2, y);

    if (t < rec.t_min || rec.t_max < t) {
        return false;
    }

    float u = f * dot(s, x);
    if (u < 0.0 || u > 1.0) {
        return false;
    }

    float v = f * dot(r.dir, y);
    if (v < 0.0 || v > 1.0 || u + v > 1.0) {
        return false;
    }

    float w = 1.0 - u - v;
    vec3 face_normal = normalize(cross(b - a, c - a));

    rec.t = t;
    rec.p = at(r, rec.t);
    vec3 outward_normal = na * w + nb * u + nc * v;
    bool front_face = dot(r.dir, face_normal) < 0.0;
    set_front_face_and_normal(rec, front_face, outward_normal);
    vec2 uv = ta * w + tb * u + tc * v;
    rec.u = uv.x;
    rec.v = uv.y;
    // rec.material = Some(self.material.clone());
    // rec.override_color = None;

    return true;
}

vec3 ray_color(in Ray r) {
    for (int i = 0; i < primitive_count; i++) {
        AttributeData position = positionData[i];
        AttributeData normal = AttributeData(vec3(0), vec3(0), vec3(0));
        AttributeData uv = AttributeData(vec3(0), vec3(0), vec3(0));

        triangle tr = triangle(position, normal, uv);

        HitRecord rec;

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

float random_in_range(vec2 co, float from, float to) {
    return from + (to - from) * rand(co); 
}

vec3 random_in_unit_disc(vec2 co) {
    vec3 p = vec3(0);
    for (int i = 0; i < 10; i++) {
        float r1 = random_in_range(vec2(co), -1.0, 1.0);
        float r2 = random_in_range(vec2(r1), -1.0, 1.0);
        p = vec3(r1, r2, 0.0);
        if (length(p) < 1.0) {
            return p;
        }
    }
    return p;
}

Ray camera_get_ray(vec2 st) {
    vec3 rd = random_in_unit_disc(st) * camera.lens_radius;
    vec3 offset = camera.u * rd.x + camera.v * rd.y;

    return Ray(
        camera.origin + offset,
        camera.lower_left_corner + st.x * camera.horizontal + st.y * camera.vertical - offset
    );
}

// main

void main() {
    uint idx = gl_GlobalInvocationID.x;

    // calculate uv
    uint x = idx % uint(image_width);
    uint y = (idx - x) / uint(image_width);

    // calculate color
    vec3 pixel_color = vec3(0);

    // antialiasing
    for (int i = 0; i < int(samples_per_pixel); i++) {
        float dp = float(i) / samples_per_pixel;

        float r1 = rand(vec2(dp));
        float r2 = rand(vec2(r1));

        float u = (float(x) + r1) / (image_width - 1);
        float vv = (float(y) + r2) / (image_height - 1);
        float v = 1.0 - vv;

        vec2 uv = vec2(u, v);

        Ray ray = camera_get_ray(uv);

        pixel_color += ray_color(ray);
    }

    pixel_color /= float(samples_per_pixel);

    colorBuffer[idx] = vec4(pixel_color, 1.0);
}