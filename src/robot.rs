use std::sync::{Arc, Mutex};
use crate::map::{Map, Cell};

pub struct Robot {
    x: usize,
    y: usize,
    role: Role,
}

pub enum Role {
    Explorer,
    Collector,
    Scientist,
}

impl Robot {
    pub fn new(x: usize, y: usize, role: Role) -> Self {
        Robot { x, y, role }
    }

    pub fn explore(&mut self, map: Arc<Mutex<Map>>) {
        let mut map = map.lock().unwrap();
        if let Some(resource) = map.collect_resource(self.x, self.y) {
            println!("Robot at ({}, {}) collected {:?}", self.x, self.y, resource);
        }
        // Move the robot randomly for the next action (to be implemented)
    }
}
