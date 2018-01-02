use glfw::{self, Key, Action};
use cgmath::Vector2;
use std::collections::HashMap;

pub struct InputManager {

    //Keyboard
    keys : HashMap<Key, Action>

}

impl InputManager {

    pub fn new() -> InputManager {
        InputManager {
            keys : HashMap::new()
        }
    }

    pub fn mutate_key(&mut self, key : Key, action : Action){
        //println!("Mutate: {:?} {:?}", key, action);
        self.keys.insert(key, action);
    }

    pub fn get_key(&self, key : &Key) -> Option<&Action> {
        let action = self.keys.get(key);
        //println!("Get: {:?} {:?}", key, action);

        action
    }

}