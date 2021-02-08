#include <iostream>
#include <vector>
#include <string>

#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "stb_image_write.h"

#include "vec3.h"

int main() {

    // Image

    int image_width = 256;
    int image_height = 256;

    // Render

    std::vector<char> pixels;

    auto size = image_width * image_height * 4;

    pixels.resize(size);

    int k = 0;

    for (int j = image_height - 1; j >= 0; --j) {
        for (int i = 0; i < image_width; ++i) {
            auto r = double(i) / (image_width - 1);
            auto g = double(j) / (image_height - 1);
            auto b = 0.25;

            int ir = static_cast<char>(255.999 * r);
            int ig = static_cast<char>(255.999 * g);
            int ib = static_cast<char>(255.999 * b);

            pixels[k + 0] = ir;
            pixels[k + 1] = ig;
            pixels[k + 2] = ib;
            pixels[k + 3] = 255;

            k += 4;
        }
    }

    int n = 4;

    std::string outputPath = "output.jpeg";

    stbi_write_jpg(outputPath.c_str(), image_width, image_height, n, pixels.data(), 0);
}