use ale_math::Vector3;

pub struct Text {
    pos: Vector3<f32>,
    text: String,
}

impl Text {
    pub fn new(pos: Vector3<f32>, text: String) -> Text {
        Text {
            pos,
            text
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn set_pos(&mut self, pos: Vector3<f32>) { self.pos = pos; }
}