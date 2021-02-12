#pragma once

#include <vector>

#include "vec3.h"

void write_color(std::vector<char> &pixels, color pixel_color, int i) {
    pixels[i + 0] = static_cast<char>(255.999 * pixel_color.x());
    pixels[i + 1] = static_cast<char>(255.999 * pixel_color.y());
    pixels[i + 2] = static_cast<char>(255.999 * pixel_color.z());
    pixels[i + 3] = (char) 255;
}
