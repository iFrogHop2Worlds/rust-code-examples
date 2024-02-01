use serde_json::Map;
use serde_json::value::Value;
use super::todo::ItemTypes;
use super::todo::structs::done::Done;
use super::todo::structs::pending::Pending;
use super::todo::traits::get::Get;
use super::todo::traits::create::Create;
use super::todo::traits::delete::Delete;
use super::todo::traits::edit::Edit;

// Accessing our traits in a standardized way for a more scalable approach.
fn process_pending(item: Pending, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.super_struct.title, state),
        "create" => item.create(&item.super_struct.title, &item.super_struct.status.to_string(), &mut state),
        "edit" => item.set_to_done(&item.super_struct.title, &mut state),
        _ => println!("command: {} not supported", command)
    }
}

fn process_done(item:Done, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.super_struct.title, state),
        "delete" => item.delete(&item.super_struct.title, state),
        "edit" => item.set_to_pending(&item.super_struct.title, &mut state),
        _ => println!("Command: {} not supported", command)
    }
}

// We can now process both our structs. This is where the scalability of
// structs comes in when designing our module. Like the commands, we
// will want to stack our structs just like we did with our traits. This is
// where our entry point comes in
//
// we can scale the access to structs by
// increasing the routes by the entry point. To appreciate this more, we
// should define our entry point, which this time is a public function
pub fn process_input(item: ItemTypes, command: String, state: &Map<String, Value>) {
    match item {
        ItemTypes::Pending(item) => process_pending(item, command, state),
        ItemTypes::Done(item) => process_done(item, command, state)
    }
}