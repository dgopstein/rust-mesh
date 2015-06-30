#version 330
layout(triangles) in;
layout(triangle_strip, max_vertices = 3) out;

in Data
{
  vec4 position;
} vdata[3];

out Data
{
  noperspective out vec3 dist;
} gdata;

void main()
{
  vec2 scale = vec2(500.0f, 500.0f); // scaling factor to make 'd' in frag shader big enough to show something
  vec2 p0 = scale * vdata[0].position.xy/vdata[0].position.w;
  vec2 p1 = scale * vdata[1].position.xy/vdata[1].position.w;
  vec2 p2 = scale * vdata[2].position.xy/vdata[2].position.w;

  vec2 v0 = p2-p1;
  vec2 v1 = p2-p0;
  vec2 v2 = p1-p0;
  float area = abs(v1.x*v2.y - v1.y*v2.x);

  gdata.dist = vec3(area/length(v0),0,0);
  gl_Position = vdata[0].position;
  EmitVertex();

  gdata.dist = vec3(0,area/length(v1),0);
  gl_Position = vdata[1].position;
  EmitVertex();

  gdata.dist = vec3(0,0,area/length(v2));
  gl_Position = vdata[2].position;
  EmitVertex();

  EndPrimitive();
}
