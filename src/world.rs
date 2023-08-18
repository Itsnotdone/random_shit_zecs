use std::any::Any;
use std::cell::UnsafeCell;
pub struct World{
    pub entities: UnsafeCell<Vec<Box<dyn Any>>>
}

impl World{
    pub fn new() -> Self{
        Self { 
            entities: UnsafeCell::new(Vec::new())
        }
    }

    pub fn add_entity(&mut self, entity: impl Any + 'static){
        unsafe{(*self.entities.get()).push(Box::new(entity))}
    }

    pub fn get<C: 'static>(&self) -> Option<&C>{
        unsafe{(*self.entities.get()).iter().find_map(|component| component.downcast_ref::<C>())}
    }

    pub unsafe fn get_mut<C: 'static>(&self) -> Option<&mut C>{
        (*self.entities.get()).iter_mut().find_map(|component| {component.downcast_mut::<C>()})
    }
}