use crate::{building::BuildingType, NumberItem};

#[derive(Debug)]
pub struct Belt;
impl BuildingType for Belt {
    fn perform_action(&self, contained_numbers: &[NumberItem]) -> Result<Option<NumberItem>, ()> {
        Ok(Some(contained_numbers[0]))
    }

    fn get_input_count(&self) -> usize {
        1
    }
}
