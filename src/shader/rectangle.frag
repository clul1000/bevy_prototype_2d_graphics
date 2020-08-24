#version 450
layout(location = 0) out vec4 o_Target;
layout(location = 0) in vec2 f_Uv;

layout(set = 1, binding = 1) uniform RectangleStyle {
    vec4 fill_color;
	vec4 border_color;
	vec2 border_width;
};

void main() {
    vec2 uv = (f_Uv - 0.5) * 2;

	o_Target = fill_color;

	if (uv.x < border_width.x - 1. || uv.y < border_width.y - 1.
		|| uv.x > 1. - border_width.x || uv.y > 1. - border_width.y) {
		o_Target = border_color;
	}
}
