FRAGMENT_TEXTURE :: #string GL
#version 330 core

in vec2 textureCoordinates;

uniform sampler2D texture1;
uniform vec4 color;

out vec4 fragmentColor;

void main() {
    fragmentColor = texture(texture1, textureCoordinates) * color;
}
GL
