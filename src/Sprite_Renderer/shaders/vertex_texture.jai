VERTEX_TEXTURE :: #string GL
#version 330 core

layout (location = 0) in vec2 xy;
layout (location = 1) in vec2 uv;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

out vec2 textureCoordinates;

void main() {
    gl_Position = projection * view * model * vec4(xy, 0, 1);
    textureCoordinates = uv;
}
GL
