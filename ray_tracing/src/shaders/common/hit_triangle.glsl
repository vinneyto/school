bool hit_triangle(in triangle tr, in out hit_record rec) {
    vec3 a = tr.position.a;
    vec3 b = tr.position.b;
    vec3 c = tr.position.c;

    vec3 na = tr.normal.a;
    vec3 nb = tr.normal.b;
    vec3 nc = tr.normal.c;

    vec2 ta = tr.uv.a.xy;
    vec2 tb = tr.uv.b.xy;
    vec2 tc = tr.uv.c.xy;

    ray r = rec.r;

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