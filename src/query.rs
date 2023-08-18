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
        unsafe{
            (*world.entities.get()).iter().for_each(|_entity| {
                if let Some(item) = T::fetch(world){
                    query_items.push(item)
                }
            });
        }
        query_items
    }
}

pub trait Queryable<'a>{
    type Item;
    fn query() -> Query<'a, Self> where Self: Sized;

    fn fetch(world: &'a World) -> Option<Self::Item>;
}

impl <'a, T: 'static>Queryable<'a> for &Entity<T>{
    type Item = &'a Entity<T>;

    fn query() -> Query<'a, Self> where Self: Sized {
        Query::new()
    }

    fn fetch(world: &'a World) -> Option<Self::Item>{
        world.get::<Entity<T>>()
    }
}

impl <'a, T: 'static>Queryable<'a> for &mut Entity<T>{
    type Item = &'a mut Entity<T>;

    fn query() -> Query<'a, Self> where Self: Sized {
        Query::new()
    }

    fn fetch(world: &'a World) -> Option<Self::Item>{
        unsafe{world.get_mut::<Entity<T>>()}
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

    fn fetch(world: &'a World) -> Option<Self::Item>{
        Some((T0::fetch(world)?, T1::fetch(world)?))
    }
}