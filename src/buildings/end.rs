use crate::{building::BuildingType, NumberItem};

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
