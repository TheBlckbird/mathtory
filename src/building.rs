use std::fmt::Debug;

use crate::NumberItem;

#[derive(Debug)]
pub struct Building {
    pub building_type: Box<dyn BuildingType>,
    pub numbers: Vec<NumberItem>,
}

impl Building {
    pub fn new(building_type: Box<dyn BuildingType>) -> Self {
        Self {
            building_type,
            numbers: Vec::new(),
        }
    }

    pub fn perform_action(&mut self) -> Result<Option<NumberItem>, ()> {
        if self.numbers.len() != self.building_type.get_input_count() {
            return Err(());
        }

        let output = self.building_type.perform_action(&self.numbers);
        self.numbers = Vec::new();
        output
    }
}

impl Default for Building {
    fn default() -> Self {
        Self {
            building_type: Box::new(End),
            numbers: Default::default(),
        }
    }
}

pub trait BuildingType: Debug {
    fn perform_action(&self, contained_numbers: &[NumberItem]) -> Result<Option<NumberItem>, ()>;
    fn get_input_count(&self) -> usize;
}

#[derive(Debug)]
pub struct Adder;
impl BuildingType for Adder {
    fn perform_action(&self, contained_numbers: &[NumberItem]) -> Result<Option<NumberItem>, ()> {
        Ok(Some(contained_numbers[0] + contained_numbers[1]))
    }

    fn get_input_count(&self) -> usize {
        2
    }
}

#[derive(Debug)]
pub struct Generator;
impl BuildingType for Generator {
    fn perform_action(&self, _contained_numbers: &[NumberItem]) -> Result<Option<NumberItem>, ()> {
        Ok(Some(1.0))
    }

    fn get_input_count(&self) -> usize {
        0
    }
}

#[derive(Debug)]
pub struct End;
impl BuildingType for End {
    fn perform_action(&self, contained_numbers: &[NumberItem]) -> Result<Option<NumberItem>, ()> {
        println!("Reached an end: {}", contained_numbers.first().unwrap());
        Ok(None)
    }

    fn get_input_count(&self) -> usize {
        1
    }
}
