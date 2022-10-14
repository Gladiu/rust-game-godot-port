use gdnative::prelude::*;
use gdnative::api::Spatial;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct Map {

}

#[gdnative::derive::methods]
impl Map {
    fn new(_owner: &Spatial) -> Self {
        Map {}
    }

    #[gdnative::derive::method]
    fn _ready(&mut self, #[base] owner: &Spatial) {
        let model = unsafe { owner.get_node("model").unwrap().assume_safe() };
        godot_print!("{:?}", model.get_children());

    }
}
