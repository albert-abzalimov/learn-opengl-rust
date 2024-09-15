#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in float a_texture_index;

out vec2 TexCoord;
out float texture_index;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    gl_Position = projection * view * model * vec4(aPos, 1.0);
    TexCoord = aTexCoord;
    texture_index = a_texture_index; 
    gl_FragColor = vec4(a_texture_index, a_texture_index, a_texture_index, 1.0);
}