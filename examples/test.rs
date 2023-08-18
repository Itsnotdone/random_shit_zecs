use zecs::{*, Queryable};

#[derive(Debug)]
struct A;

#[derive(Debug)]
struct B;

fn main(){
    let mut world = World::new();
    let mut entity_a = Entity::<A>::new();
    let mut entity_b = Entity::<B>::new();

    entity_a.add_component(40);
    entity_b.add_component(80);

    world.add_entity(entity_a);
    world.add_entity(entity_b);

    let query = <(&mut Entity<A>, &mut Entity<B>)>::query();

    for (entity_a, entity_b) in query.iter(&world){
        (*entity_a.get_mut::<i32>().unwrap()) *= 2;
        (*entity_b.get_mut::<i32>().unwrap()) *= 2;
    }

    for (entity_a, entity_b) in query.iter(&world){
        println!("{}", entity_a.get::<i32>().unwrap());
        println!("{}", entity_b.get::<i32>().unwrap());
    }
}