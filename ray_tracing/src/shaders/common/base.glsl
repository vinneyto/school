struct attribute_data {
    vec3 a;
    vec3 b;
    vec3 c;
};

struct triangle {
   attribute_data position;
   attribute_data normal;
   attribute_data uv;
};

struct ray {
    vec3 origin;
    vec3 dir;
};

struct hit_record {
    ray r;
    float t_min;
    float t_max;
    vec3 p;
    vec3 normal;
    float t;
    float u;
    float v;
    bool front_face;
};

vec3 at(ray r, float t) {
    return r.origin + r.dir * t;
}

void set_front_face_and_normal(in out hit_record rec, bool front_face, vec3 outward_normal) {
    rec.front_face = front_face;
    rec.normal = rec.front_face ? outward_normal : -outward_normal;
}