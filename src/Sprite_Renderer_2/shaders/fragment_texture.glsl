#version 330 core

in vec2 textureCoordinates;
in vec4 color;

uniform sampler2D texture1;

out vec4 fragmentColor;

void main() {
    fragmentColor = texture(texture1, textureCoordinates) * color;
}
