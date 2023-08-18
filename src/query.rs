use std::marker::PhantomData;
use crate::{
    World, Entity
};

pub struct Query<'a, T: Queryable<'a>>{
    _marker: PhantomData<&'a T>
}

impl <'a, T: Queryable<'a, Item = T>>Query<'a, T>{
    pub fn new() -> Self{
        Self { 
            _marker: PhantomData 
        }
    }

    pub fn iter(&self, world: &'a World) -> impl IntoIterator<Item = T>{
        let mut query_items = Vec::new();
        world.entities.iter().for_each(|entity| {
            if let Some(item) = T::fetch(entity){
                query_items.push(item)
            }
        });
        
        query_items
    }
}

pub trait Queryable<'a>{
    type Item;
    fn query() -> Query<'a, Self> where Self: Sized;

    fn fetch(entity: &'a Entity) -> Option<Self::Item>;
}

impl <'a, T: 'static>Queryable<'a> for &T{
    type Item = &'a T;

    fn query() -> Query<'a, Self> where Self: Sized {
        Query::new()
    }

    fn fetch(entity: &'a Entity) -> Option<Self::Item>{
        entity.get::<T>()
    }
}

impl <'a, T: 'static>Queryable<'a> for &mut T{
    type Item = &'a mut T;

    fn query() -> Query<'a, Self> where Self: Sized {
        Query::new()
    }

    fn fetch(entity: &'a Entity) -> Option<Self::Item>{
        unsafe{entity.get_mut::<T>()}
    }
}

impl <'a, T0, T1>Queryable<'a> for (T0, T1)
where
    T0: Queryable<'a, Item = T0>,
    T1: Queryable<'a, Item = T1>
{
    type Item = (T0, T1);

    fn query() -> Query<'a, Self> where Self: Sized {
        Query::new()
    }

    fn fetch(entity: &'a Entity) -> Option<Self::Item>{
        Some((T0::fetch(entity)?, T1::fetch(entity)?))
    }
}