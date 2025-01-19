use godot::classes::{INode, Node};
use godot::global::godot_print;
use godot::obj::Base;
use godot::prelude::{godot_api, GodotClass};
use serde_json::Value;

#[derive(GodotClass)]
#[class(base=Node)]
struct OSMProcessor {
    base: Base<Node>
}

type Vector3 = (f64, f64, f64);

#[godot_api]
impl INode for OSMProcessor {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Hello, world!");

        Self {
            base,
        }
    }
}

#[godot_api]
impl OSMProcessor {
    #[func]
    fn process(path: String) {
        let json = load_json(path);
        godot_print!("{}", json["version"])
    }

}

fn load_json(path: String) -> Value {
    serde_json::from_str(std::fs::read_to_string(path).unwrap().as_str()).expect("Error parsing JSON")
}