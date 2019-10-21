#version 430 core

in vec2 fs_TexCoord;
out vec4 out_Color;

uniform sampler2D _Tex;

void main() {
    out_Color = vec4(texture2D(_Tex, fs_TexCoord).rgb, 1.0);
}
