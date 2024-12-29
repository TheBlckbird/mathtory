use crate::{building::BuildingType, NumberItem};

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
