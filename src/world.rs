use std::any::{Any, TypeId};
use std::cell::UnsafeCell;

use crate::Entity;
pub struct World{
    pub entities: Vec<Entity>
}

impl World{
    pub fn new() -> Self{
        Self { 
            entities: Vec::new()
        }
    }

    pub fn add_entity(&mut self, entity: Entity){
        self.entities.push(entity);
    }
}