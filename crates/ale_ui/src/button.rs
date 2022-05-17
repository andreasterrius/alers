use ale_input::Input;
use ale_math::color::Color;

pub struct Button {
    idle_color: Color,
    enter_color: Color,
    click_color: Color,

    is_disable: bool,
}

impl Button {
    pub fn new(idle_color: Color,
               enter_color: Color,
               click_color: Color) -> Button {
        return Button {
            idle_color,
            enter_color,
            click_color,
            is_disable: false
        };
    }

    pub fn input(input : &Input){

    }

    pub fn render() {

    }
}