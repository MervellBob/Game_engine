#version 330 core

    layout (location = 0) in vec3 pos;
    layout (location = 1) in vec3 color;

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;

    out vec3 ourColor;
    void main() {
        gl_Position = projection * view * model * vec4(pos.x, pos.y, pos.z, 1.0);
        ourColor = color;
    }
