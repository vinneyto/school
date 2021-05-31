vec2 uv;

{
    uint x = idx % uint(opData.image_width);
    uint y = (idx - x) / uint(opData.image_width);

    float u = float(x) / (opData.image_width - 1);
    float vv = float(y) / (opData.image_height - 1);
    float v = 1.0 - vv;

    uv.x = u;
    uv.y = v;
}

