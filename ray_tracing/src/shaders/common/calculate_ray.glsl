ray r;

{
    // Image
    float aspect_ratio = 16.0 / 9.0;

    // Camera
    float viewport_height = 2.0;
    float viewport_width = aspect_ratio * viewport_height;
    float focal_length = 1.0;

    vec3 origin = vec3(0.0, 0.0, 0.0);
    vec3 horizontal = vec3(viewport_width, 0.0, 0.0);
    vec3 vertical = vec3(0.0, viewport_height, 0.0);
    vec3 lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - vec3(0.0, 0.0, focal_length);

    r.origin = origin;
    r.dir = lower_left_corner + uv.x * horizontal + uv.y * vertical - origin;
}

