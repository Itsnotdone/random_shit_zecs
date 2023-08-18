use zecs::{*, Queryable};

#[derive(Debug)]
struct A;

#[derive(Debug)]
struct B;

#[test]
fn ub_test(){
    let mut world = World::new();
    let mut entity_a = Entity::new();
    let mut entity_b = Entity::new();

    entity_a.add_component(40);
    entity_a.add_component("cos".to_string());
    entity_b.add_component(80);

    world.add_entity(entity_a);
    world.add_entity(entity_b);

    let query = <(&mut i32, &String)>::query();

    for (number, _) in query.iter(&world){
        *number *= 2;
    }

    for (number, string) in query.iter(&world){
        println!("{}", string);
    }
}