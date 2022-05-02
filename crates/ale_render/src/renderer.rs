pub trait Renderer {
    // 2D
    fn render_text();

    // Mesh
    fn render_pbr_mesh();
    fn render_wire_mesh();

    // Plane
    fn render_textured_plane();

    // Debug
    fn render_debug_line3d();
    fn render_debug_point3d();
}