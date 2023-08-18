use std::any::Any;
use std::cell::UnsafeCell;

#[derive(Debug)]
pub struct Entity{
    components: UnsafeCell<Vec<Box<dyn Any>>>,
}

impl Entity{
    pub fn new() -> Self{
        Self { 
            components: UnsafeCell::new(Vec::new()),
        }
    }

    pub fn add_component(&mut self, component: impl Any + 'static){
        self.components.get_mut().push(Box::new(component));
    }

    pub fn get<C: 'static>(&self) -> Option<&C>{
        unsafe{(*self.components.get()).iter().find_map(|component| component.downcast_ref::<C>())}
    }

    pub unsafe fn get_mut<C: 'static>(&self) -> Option<&mut C>{
        
        (*self.components.get()).iter_mut().find_map(|component| {component.downcast_mut::<C>()})
    }
}