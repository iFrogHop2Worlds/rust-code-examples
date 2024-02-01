use std::fmt;
use crate::todo::enums::TaskStatus::DONE;
use crate::todo::enums::TaskStatus::PENDING;

pub enum TaskStatus {
    DONE,
    PENDING,
}
 impl fmt::Display for TaskStatus {
    
    fn fmt(&self, f: &mut fmt::Formatter) ->  fmt::Result {
        match &self {
            &self::DONE => {write!(f, "DONE")},
            &self::PENDING => {write!(f, "PENDING")}
        }
    }
 }

 impl TaskStatus {
    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("input {} not suppoerted", input_string)
        }
    }
 }