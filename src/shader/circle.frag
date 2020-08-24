#version 450
layout(location = 0) out vec4 o_Target;
layout(location = 0) in vec2 f_Uv;

layout(set = 1, binding = 1) uniform CircleStyle {
    vec4 fill_color;
	vec4 border_color;
	float border_width;
};

void main() {
    vec2 uv = (f_Uv - 0.5) * 2;
	float square_len = uv.x * uv.x + uv.y * uv.y;
	if (square_len < 1.) {
		o_Target = fill_color;
		if (square_len > 1 - border_width) {
			o_Target = border_color;
		}
	} else {
		discard;
	}
}
