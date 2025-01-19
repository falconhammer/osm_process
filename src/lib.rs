use godot::prelude::*;

mod osm_processor;

struct OSMProcess;

#[gdextension]
unsafe impl ExtensionLibrary for OSMProcess {
}