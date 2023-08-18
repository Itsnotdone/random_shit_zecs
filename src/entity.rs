use std::any::Any;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Entity<T>{
    components: Vec<Box<dyn Any>>,
    _marker: PhantomData<T>
}

impl <T>Entity<T>{
    pub fn new() -> Self{
        Self { 
            components: Vec::new(), 
            _marker: PhantomData
        }
    }

    pub fn add_component(&mut self, component: impl Any + 'static){
        self.components.push(Box::new(component));
    }

    pub fn get<C: 'static>(&self) -> Option<&C>{
        self.components.iter().find_map(|component| component.downcast_ref::<C>())
    }

    pub fn get_mut<C: 'static>(&mut self) -> Option<&mut C>{
        self.components.iter_mut().find_map(|component| {component.downcast_mut::<C>()})
    }
}