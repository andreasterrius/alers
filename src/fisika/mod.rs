use cgmath::Vector2;

pub trait BoxCollider2D {
    fn get_world_position(&self) -> Vector2<f32>;

    fn get_size(&self) -> Vector2<f32>;
}

pub trait CircleCollider2D {
    fn get_world_position(&self) -> Vector2<f32>;

    fn get_radius(&self) -> f32;
}


fn aabb_axis_overlap( pos1 : f32, size1 : f32, pos2 : f32, size2: f32) -> bool {
    pos1 < pos2 + size2 &&
    pos1 + size1 > pos2
}

pub fn aabb_collision_box_box<Box1, Box2>(box_collider1: &Box1, box_collider2: &Box2) -> bool
    where Box1 : BoxCollider2D, Box2 : BoxCollider2D
{
    aabb_axis_overlap(box_collider1.get_world_position().x, box_collider1.get_size().x,
        box_collider2.get_world_position().x, box_collider2.get_size().x)
    &&
    aabb_axis_overlap(box_collider1.get_world_position().y, box_collider1.get_size().y,
                          box_collider2.get_world_position().y, box_collider2.get_size().y)
}

pub fn aabb_collission_box_circle<Box, Circle>(box_collider1: &Box, circle_collider: &Circle) -> bool
    where Box : BoxCollider2D, Circle : CircleCollider2D
{
    unimplemented!()
}
