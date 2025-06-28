#version 330 core

layout (location = 0) in vec2 xy;
layout (location = 1) in vec2 uv;
layout (location = 2) in vec4 rgba;

uniform mat4 projection;
// uniform mat4 view; @Todo uncomment this when we have camera
uniform mat4 model_transforms[3];

out vec2 textureCoordinates;
out vec4 color;

void main() {
    gl_Position = projection * /* view * */ model_transforms[gl_InstanceID] * vec4(xy, 0, 1);
    textureCoordinates = uv;
    color = rgba;
}
