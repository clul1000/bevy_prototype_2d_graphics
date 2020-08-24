#version 450
layout(location = 0) out vec4 o_Target;
layout(location = 0) in vec2 f_Uv;

layout(set = 1, binding = 1) uniform LineStyle {
	vec4 color;
	float width;
	float height;
	float stroke;
};

void main() {
	vec2 pos = vec2(f_Uv.x * width, f_Uv.y * height);

	vec2 start = vec2(stroke, height / 2.);
	vec2 end = vec2(width - stroke, height / 2.);

	vec2 p;

	if (dot(pos - start, end - start) > 0) {
		if (dot(pos - end, start - end) > 0) {
			o_Target = color;
			return;
		} else {
			p = end;
		}
	} else {
		p = start;
	}

	vec2 dist = pos - p;
	float squared_len = dist.x * dist.x + dist.y * dist.y;

	if (squared_len < stroke * stroke) {
		o_Target = color;
	} else {
		discard;
	}
}
