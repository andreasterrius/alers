use ale_math::rect::Rect;

pub struct Viewport {
    rect : Rect
}

impl Viewport {
    pub fn new(rect: Rect) -> Viewport {
        return Viewport {
            rect
        }
    }
}