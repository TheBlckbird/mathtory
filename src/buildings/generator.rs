use rand::Rng;

use crate::{building::BuildingType, NumberItem};

#[derive(Debug)]
pub struct Generator;
impl BuildingType for Generator {
    fn perform_action(&self, _contained_numbers: &[NumberItem]) -> Result<Option<NumberItem>, ()> {
        Ok(Some(rand::thread_rng().gen_range(0..10) as NumberItem))
    }

    fn get_input_count(&self) -> usize {
        0
    }
}
