use std::fmt::Debug;

use crate::{buildings::end::End, NumberItem};

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
