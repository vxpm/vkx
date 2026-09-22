#version 460

const vec2 positions[3] = vec2[](
    vec2( 0.0, +1.0),
    vec2(-1.0, -1.0),
    vec2(+1.0, -1.0)
);

const vec4 colors[3] = vec4[](
    vec4(1.0, 0.0, 0.0, 1.0),
    vec4(0.0, 1.0, 0.0, 1.0),
    vec4(0.0, 0.0, 1.0, 1.0)
);

layout (location = 0) out vec4 out_color;

void main()
{
    out_color = colors[gl_VertexID];
    vec2 position = positions[gl_VertexID];
    gl_Position = vec4(position, 0.0, 1.0);
}
