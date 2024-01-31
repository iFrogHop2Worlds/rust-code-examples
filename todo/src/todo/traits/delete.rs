use serde_json::Map;
use serde_json::value::Value;
use crate::state::write_to_file;

pub trait Delete {
    fn delete(&self, title: &str, state: Map<String, Value>) {
        state.remove(title);
        write_to_file("./state.json", state);
        println!("Deleted: {}", title);
    }
}