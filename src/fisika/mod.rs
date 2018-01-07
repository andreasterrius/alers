use cgmath::Vector2;
use cgmath::prelude::*;
use math::*;

pub trait BoxCollider2D {
    fn worldpos(&self) -> Vector2<f32>;

    fn size(&self) -> Vector2<f32>;
}

pub trait CircleCollider2D {
    fn worldpos(&self) -> Vector2<f32>;

    fn radius(&self) -> f32;
}


fn aabb_axis_overlap( pos1 : f32, size1 : f32, pos2 : f32, size2: f32)
    -> bool
{
    let end1 = pos1 + size1;
    let end2 = pos2 + size2;
    pos1 < end2 && end1 > pos2
}

pub fn aabb_collision_box_box<Box1, Box2>(box_collider1: &Box1, box_collider2: &Box2) -> bool
    where Box1 : BoxCollider2D, Box2 : BoxCollider2D
{
    aabb_axis_overlap(box_collider1.worldpos().x, box_collider1.size().x,
                      box_collider2.worldpos().x, box_collider2.size().x)
    &&
    aabb_axis_overlap(box_collider1.worldpos().y, box_collider1.size().y,
                      box_collider2.worldpos().y, box_collider2.size().y)
}

pub fn aabb_collision_box_circle<Box, Circle>(box_collider: &Box,
                                              circle_collider: &Circle) -> Option<(Vector2<f32>, f32)>
    where Box : BoxCollider2D, Circle : CircleCollider2D
{
    //find difference vector
    let diff_vec = get_center_pos_circle(circle_collider) - get_center_pos_box(box_collider);

    //local space
    let box_closest_point_to_circle = Vector2::new(
        clamp(diff_vec.x, -box_collider.size().x/2.0, box_collider.size().x/2.0),
        clamp(diff_vec.y, -box_collider.size().y/2.0, box_collider.size().y/2.0)
    );

    let circle_to_box_closest_point_magnitude = (diff_vec - box_closest_point_to_circle).magnitude();
    let diff = circle_collider.radius() - circle_to_box_closest_point_magnitude;

    if circle_to_box_closest_point_magnitude < circle_collider.radius() {
        return Some((box_closest_point_to_circle, diff));
    }

    None
}

#[derive(Debug, Clone)]
pub enum BoxGeneralArea {
    Top,
    Right,
    Bottom,
    Left,
}


pub fn determine_point_in_box_general<Box>(box_collider: &Box, point_in_box : Vector2<f32>) -> BoxGeneralArea
    where Box : BoxCollider2D
{
    let k_eps = 0.001;

    let directions : [(Vector2<f32>, BoxGeneralArea); 4] = [
        (Vector2::new(0.0, 1.0), BoxGeneralArea::Top),
        (Vector2::new(1.0, 0.0), BoxGeneralArea::Right),
        (Vector2::new(0.0, -1.0), BoxGeneralArea::Bottom),
        (Vector2::new(-1.0, 0.0), BoxGeneralArea::Left)
    ];

    let mut max = 0.0;
    let mut best_match = BoxGeneralArea::Left;
    for &(ref direction, ref label) in directions.iter() {
        let norm_point = Vector2::new(point_in_box.x / (box_collider.size().x + 5.0) / 2.0,
            point_in_box.y / box_collider.size().y / 2.0);
        let dot_product = direction.dot(norm_point);
        if dot_product > max {
            max = dot_product;
            best_match = label.clone();
        }
    }

    best_match
}

pub fn get_center_pos_box<Box>(box_collider1: &Box) -> Vector2<f32>
    where Box : BoxCollider2D
{
    box_collider1.worldpos() + (box_collider1.size()/2.0)
}

pub fn get_center_pos_circle<Circle>(circle_collider1 : &Circle) -> Vector2<f32>
    where Circle : CircleCollider2D
{
    circle_collider1.worldpos() + (Vector2::from_value(circle_collider1.radius()))
}