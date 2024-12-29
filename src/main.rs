use std::{
    collections::VecDeque,
    io::{stdin, stdout, Write},
};

use building::Building;
use buildings::{
    adder::Adder, divider::Divider, end::End, generator::Generator, multiplicator::Multiplicator,
    subtractor::Subtractor,
};
use petgraph::{
    dot::{Config, Dot},
    visit::Bfs,
    Direction, Graph,
};

mod building;
mod buildings;

pub type NumberItem = f32;

fn main() {
    let mut factory = Graph::<Building, ()>::new();

    let generator1 = factory.add_node(Building::new(Box::new(Generator)));
    let generator2 = factory.add_node(Building::new(Box::new(Generator)));
    let generator3 = factory.add_node(Building::new(Box::new(Generator)));
    let generator4 = factory.add_node(Building::new(Box::new(Generator)));
    let generator5 = factory.add_node(Building::new(Box::new(Generator)));

    let adder = factory.add_node(Building::new(Box::new(Adder)));
    let subtractor = factory.add_node(Building::new(Box::new(Subtractor)));
    let multiplicator = factory.add_node(Building::new(Box::new(Multiplicator)));
    let divider = factory.add_node(Building::new(Box::new(Divider)));
    let end = factory.add_node(Building::new(Box::new(End)));

    factory.extend_with_edges([
        (generator1, adder),
        (generator2, adder),
        (generator3, subtractor),
        (generator4, subtractor),
        (adder, multiplicator),
        (subtractor, multiplicator),
        (multiplicator, divider),
        (generator5, divider),
        (divider, end),
    ]);

    //

    let mut needed_inputs = VecDeque::new();

    loop {
        println!("{:?}", Dot::with_config(&factory, &[Config::EdgeNoLabel]));

        factory.reverse();
        let mut bfs = Bfs::new(&factory, end);

        while let Some(node_index) = bfs.next(&factory) {
            let inputted_building_index = needed_inputs.pop_front();

            let input_buildings_count = factory
                .edges_directed(node_index, Direction::Outgoing)
                .count();

            let building = &mut factory[node_index];

            for _ in 0..input_buildings_count {
                needed_inputs.push_back(node_index);
            }

            let maybe_result = match building.perform_action() {
                Ok(maybe_result) => maybe_result,
                Err(_) => continue,
            };
            let result = match maybe_result {
                Some(result) => result,
                None => continue,
            };

            let inputted_building_index = match inputted_building_index {
                Some(building) => building,
                None => continue,
            };

            let inputted_building = &mut factory[inputted_building_index];
            if inputted_building.building_type.get_input_count() > inputted_building.numbers.len() {
                inputted_building.numbers.push(result);
            }
        }

        factory.reverse();

        let mut buf = String::new();

        print!("Press enter to tick...");
        stdout().flush().unwrap();
        stdin().read_line(&mut buf).unwrap();
    }
}
