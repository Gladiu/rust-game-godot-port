use gdnative::prelude::*;

mod map;

mod start;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<map::Map>();
    handle.add_class::<start::Start>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);
