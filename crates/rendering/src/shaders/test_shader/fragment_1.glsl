#version 330 core
    out vec4 final_color;
    in vec3 ourColor;

    void main() {
        final_color = vec4(ourColor, 1.0);
    }