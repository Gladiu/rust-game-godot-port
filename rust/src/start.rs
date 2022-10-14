use gdnative::prelude::*;
use gdnative::api::Node;


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ManageErrs {
    CouldNotMakeInstance,
    RootClassNotSpatial(String),
}

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Start{
    map_scene: Option<Ref<PackedScene, ThreadLocal>>,

}

#[methods]
impl Start{
    fn new(_owner: &Node) -> Self {
        godot_print!("Hello World!");
        Start{
            map_scene: None,
        }
    }

    #[gdnative::derive::method]
    fn _ready(&mut self){

        self.map_scene = load_scene("res://Map.tscn");
        match &self.map_scene {
            Some(_scene) => godot_print!("Loaded child scene successfully!"),
            None => godot_print!("Could not load child scene. Check name."),
        }

    }
}


pub fn load_scene(path: &str) -> Option<Ref<PackedScene, ThreadLocal>> {
    let scene = load::<PackedScene>(path)?;
    let scene = unsafe { scene.assume_thread_local() };
    Some(scene)
}

/// Root here is needs to be the same type (or a parent type) of the node that you put in the child
///   scene as the root. For instance Spatial is used for this example.
fn instance_scene<Root>(scene: &PackedScene) -> Result<Ref<Root, Unique>, ManageErrs>
where
    Root: gdnative::object::GodotObject<Memory = ManuallyManaged> + SubClass<Node>,
{
    let instance = scene
        .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
        .ok_or(ManageErrs::CouldNotMakeInstance)?;
    let instance = unsafe { instance.assume_unique() };

    instance
        .try_cast::<Root>()
        .map_err(|instance| ManageErrs::RootClassNotSpatial(instance.name().to_string()))
}
