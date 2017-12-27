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
                                              circle_collider: &Circle) -> Option<Vector2<f32>>
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

    if circle_to_box_closest_point_magnitude < circle_collider.radius() {
        return Some(box_closest_point_to_circle + box_collider.size()/2.0); //convert center to top left
    }

    None
}

#[derive(Debug)]
pub enum BoxGeneralArea {
    TopLeft,
    Left,
    BotLeft,

    Top,
    Bot,

    TopRight,
    Right,
    BotRight
}


pub fn determine_point_in_box_general<Box>(box_collider: &Box, point_in_box : Vector2<f32>) -> Option<BoxGeneralArea>
    where Box : BoxCollider2D
{
    let slice_size = 8.0;
    let slice_x = box_collider.size().x / slice_size;
    let slice_y = box_collider.size().y / 4.0;

    //left
    if point_in_box.x > 0.0 && point_in_box.x < slice_x {
        //top left
        if point_in_box.y > 0.0 && point_in_box.y < slice_y {
            return Some(BoxGeneralArea::TopLeft);
        }
        //bot left
        else if point_in_box.y > box_collider.size().y - slice_y
            && point_in_box.y < box_collider.size().y
        {
            return Some(BoxGeneralArea::BotLeft);
        }
        else {
            return Some(BoxGeneralArea::Left);
        }
    }

    //right
    if point_in_box.x > box_collider.size().x - slice_x &&
        point_in_box.x < box_collider.size().x
    {
        //top left
        if point_in_box.y > 0.0 && point_in_box.y < slice_y {
           return Some(BoxGeneralArea::TopRight);
        }
        //bot left
        else if point_in_box.y > box_collider.size().y - slice_y
            && point_in_box.y < box_collider.size().y
        {
            return Some(BoxGeneralArea::BotRight);
        }
        else {
            return Some(BoxGeneralArea::Right);
        }
    }

    if point_in_box.y > box_collider.size().y / 2.0 {
        return Some(BoxGeneralArea::Top);
    } else if point_in_box.y < box_collider.size(). y / 2.0 {
        return Some(BoxGeneralArea::Bot);
    }

    None
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